// handlers/auth.rs - VERSIÓN COMPLETA CON FUNCIÓN 'me'

use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use serde_json::json;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{RegisterUserRequest, LoginRequest, LoginResponse, UserRole, UserInfo};
use crate::utils::create_jwt;

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse> {
    println!("🔐 Recibiendo solicitud de registro para: {}", req.email);
    
    // ✅ Validación manual básica
    if req.email.is_empty() || req.password.is_empty() || req.first_name.is_empty() || req.last_name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Email, contraseña, nombre y apellido son requeridos"
        })));
    }

    // ✅ Validar formato de email básico
    if !req.email.contains('@') || !req.email.contains('.') {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Formato de email inválido"
        })));
    }

    // ✅ Validar longitud de contraseña
    if req.password.len() < 8 {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "La contraseña debe tener al menos 8 caracteres"
        })));
    }

    // ✅ Verificar si el usuario ya existe
    let existing_user = sqlx::query!("SELECT id FROM users WHERE email = $1", req.email)
        .fetch_optional(pool.as_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(json!({
                "error": "El email ya está registrado"
            })));
        }
        Ok(None) => {
            // Usuario no existe, continuar
        }
        Err(e) => {
            eprintln!("❌ Error verificando usuario existente: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Error interno del servidor"
            })));
        }
    }

    // ✅ Hash de la contraseña
    let password_hash = match hash(req.password.as_bytes(), DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("❌ Error hasheando contraseña: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Error procesando contraseña"
            })));
        }
    };

    // ✅ Convertir role string a enum
    let role_enum = match req.role.to_lowercase().as_str() {
        "admin" => UserRole::Admin,
        "hotel_owner" | "hotelowner" => UserRole::HotelOwner,
        "business_owner" | "businessowner" => UserRole::BusinessOwner,
        "customer" => UserRole::Customer,
        _ => UserRole::Customer, // Default
    };

    // ✅ Convertir enum a string para la BD
    let role_str = match role_enum {
        UserRole::Admin => "admin",
        UserRole::HotelOwner => "hotel_owner",
        UserRole::BusinessOwner => "business_owner",
        UserRole::Customer => "customer",
    };

    // ✅ Insertar usuario en la base de datos
    let user_result = sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name, phone, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, first_name, last_name, phone, role, created_at
        "#,
        req.email,
        password_hash,
        req.first_name,
        req.last_name,
        req.phone,
        role_str
    )
    .fetch_one(pool.as_ref())
    .await;

    let user_record = match user_result {
        Ok(user) => user,
        Err(e) => {
            eprintln!("❌ Error insertando usuario: {}", e);
            
            // Verificar si es error de duplicado
            if e.to_string().contains("duplicate key") || e.to_string().contains("unique constraint") {
                return Ok(HttpResponse::Conflict().json(json!({
                    "error": "El email ya está registrado"
                })));
            }
            
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Error al crear usuario"
            })));
        }
    };

    // ✅ Crear información de usuario
    let user_info = UserInfo {
        id: user_record.id,
        email: user_record.email,
        role: role_enum,
        first_name: user_record.first_name,
        last_name: user_record.last_name,
        phone: user_record.phone,
    };

    // ✅ Generar JWT token
    let token = match create_jwt(user_info.id, &user_info.email, role_str) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("❌ Error generando token: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Error generando token de autenticación"
            })));
        }
    };

    let response = LoginResponse {
        token,
        user: user_info,
    };

    println!("✅ Usuario registrado exitosamente: {}", response.user.email);
    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    println!("🔐 Intento de login para: {}", req.email);

    // ✅ Buscar usuario en la base de datos
    let user_result = sqlx::query!(
        "SELECT id, email, password_hash, first_name, last_name, phone, role FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(pool.as_ref())
    .await;

    match user_result {
        Ok(Some(user_record)) => {
            // ✅ Verificar contraseña
            let password_valid = match verify(&req.password, &user_record.password_hash) {
                Ok(valid) => valid,
                Err(e) => {
                    eprintln!("❌ Error verificando contraseña: {}", e);
                    return Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Error de autenticación"
                    })));
                }
            };

            if !password_valid {
                return Ok(HttpResponse::Unauthorized().json(json!({
                    "error": "Credenciales inválidas"
                })));
            }

            // ✅ Convertir string role de la BD a enum
            let role_enum = match user_record.role.as_str() {
                "admin" => UserRole::Admin,
                "hotel_owner" => UserRole::HotelOwner,
                "business_owner" => UserRole::BusinessOwner,
                "customer" => UserRole::Customer,
                _ => UserRole::Customer,
            };

            let user_info = UserInfo {
                id: user_record.id,
                email: user_record.email,
                role: role_enum,
                first_name: user_record.first_name,
                last_name: user_record.last_name,
                phone: user_record.phone,
            };

            // ✅ Convertir enum a string para JWT
            let role_str = match user_info.role {
                UserRole::Admin => "admin",
                UserRole::HotelOwner => "hotel_owner",
                UserRole::BusinessOwner => "business_owner",
                UserRole::Customer => "customer",
            };

            let token = match create_jwt(user_info.id, &user_info.email, role_str) {
                Ok(token) => token,
                Err(e) => {
                    eprintln!("❌ Error generando token: {}", e);
                    return Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Error generando token de autenticación"
                    })));
                }
            };

            let response = LoginResponse {
                token,
                user: user_info,
            };

            println!("✅ Login exitoso para: {}", req.email);
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            Ok(HttpResponse::Unauthorized().json(json!({
                "error": "Credenciales inválidas"
            })))
        }
        Err(e) => {
            eprintln!("❌ Error buscando usuario: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Error interno del servidor"
            })))
        }
    }
}

// ✅ FUNCIÓN 'me' IMPLEMENTADA CORRECTAMENTE
// Esta función requiere que el usuario esté autenticado y devuelve sus datos
pub async fn me(user: UserInfo) -> Result<HttpResponse> {
    println!("👤 Solicitud de información de usuario: {}", user.email);
    
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "phone": user.phone,
            "role": match user.role {
                UserRole::Admin => "admin",
                UserRole::HotelOwner => "hotel_owner", 
                UserRole::BusinessOwner => "business_owner",
                UserRole::Customer => "customer",
            }
        },
        "message": "Información de usuario obtenida exitosamente"
    })))
}