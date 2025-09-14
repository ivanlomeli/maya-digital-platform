mod models;
mod handlers;
mod middleware;
mod utils;

use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use handlers::{
    register, login, me, 
    get_pending_hotels, approve_hotel, reject_hotel, get_all_hotels, get_admin_stats, 
    create_hotel, get_my_hotels, update_hotel, delete_hotel, get_hotel_detail,
    create_booking, get_my_bookings, cancel_booking,
    get_admin_metrics, get_admin_businesses, get_admin_bookings, get_search_analytics,
    get_hotel_bookings,
    get_all_approved_hotels,
    // Handlers para negocios
    create_business, get_my_businesses, get_business_detail, 
    update_business, delete_business,
    // 🔥 NUEVOS HANDLERS PARA ADMINISTRADOR DE NEGOCIOS
    get_pending_businesses, approve_business, reject_business
};

#[get("/api/restaurantes")]
async fn get_restaurantes() -> impl Responder {
    let restaurantes = vec![
        serde_json::json!({
            "id": 1,
            "name": "Corazón de Jade",
            "specialty": "Cocina de Autor",
            "location": "Campeche",
            "image": "https://images.unsplash.com/photo-1552566626-52f8b828add9?q=80&w=2070&auto=format&fit=crop"
        }),
        serde_json::json!({
            "id": 2,
            "name": "La Ceiba",
            "specialty": "Mariscos Frescos",
            "location": "Chetumal",
            "image": "https://images.unsplash.com/photo-1517248135467-4c7edcad34c4?q=80&w=2070&auto=format&fit=crop"
        }),
        serde_json::json!({
            "id": 3,
            "name": "El Fogón del Jaguar",
            "specialty": "Carnes y Tradición",
            "location": "Valladolid",
            "image": "https://images.unsplash.com/photo-1414235077428-338989a2e8c0?q=80&w=2070&auto=format&fit=crop"
        })
    ];
    HttpResponse::Ok().json(restaurantes)
}

#[get("/api/experiencias")]
async fn get_experiencias() -> impl Responder {
    let experiencias = vec![
        serde_json::json!({
            "id": 1,
            "type": "tour",
            "name": "Tour a Chichén Itzá",
            "price": 1200,
            "image": "https://images.unsplash.com/photo-1528181304800-259b08848526?q=80&w=2070&auto=format&fit=crop"
        }),
        serde_json::json!({
            "id": 2,
            "type": "caballos",
            "name": "Paseo a Caballo",
            "price": 850,
            "image": "https://images.unsplash.com/photo-1599059813005-3603a5603703?q=80&w=1974&auto=format&fit=crop"
        }),
        serde_json::json!({
            "id": 3,
            "type": "cenote",
            "name": "Nado en Cenote Sagrado",
            "price": 450,
            "image": "https://images.unsplash.com/photo-1627907222543-4111d6946196?q=80&w=1965&auto=format&fit=crop"
        })
    ];
    HttpResponse::Ok().json(experiencias)
}

#[get("/api/productos")]
async fn get_productos() -> impl Responder {
    let productos = vec![
        serde_json::json!({
            "id": 1,
            "name": "Huipil Ceremonial",
            "artisan": "Elena Poot",
            "price": 1800,
            "category": "textil",
            "img": "https://images.unsplash.com/photo-1620921207299-b37993505b12?q=80&w=1964&auto=format&fit=crop",
            "desc": "Tejido a mano con técnicas ancestrales, este huipil representa la cosmovisión maya en cada uno de sus hilos."
        }),
        serde_json::json!({
            "id": 2,
            "name": "Vasija de Sac-bé",
            "artisan": "Mateo Cruz",
            "price": 950,
            "category": "ceramica",
            "img": "https://images.unsplash.com/photo-1578899223131-a7isea110323?q=80&w=1887&auto=format&fit=crop",
            "desc": "Cerámica de alta temperatura pintada a mano con pigmentos naturales, ideal para decoración."
        }),
    ];
    HttpResponse::Ok().json(productos)
}

#[get("/api/productos/{id}")]
async fn get_producto_by_id(path: web::Path<u32>) -> impl Responder {
    let product_id = path.into_inner();
    let productos = vec![
        serde_json::json!({
            "id": 1,
            "name": "Huipil Ceremonial",
            "artisan": "Elena Poot",
            "price": 1800,
            "category": "textil",
            "img": "https://images.unsplash.com/photo-1620921207299-b37993505b12?q=80&w=1964&auto=format&fit=crop",
            "desc": "Tejido a mano con técnicas ancestrales, este huipil representa la cosmovisión maya en cada uno de sus hilos."
        }),
        serde_json::json!({
            "id": 2,
            "name": "Vasija de Sac-bé",
            "artisan": "Mateo Cruz",
            "price": 950,
            "category": "ceramica",
            "img": "https://images.unsplash.com/photo-1578899223131-a7isea110323?q=80&w=1887&auto=format&fit=crop",
            "desc": "Cerámica de alta temperatura pintada a mano con pigmentos naturales, ideal para decoración."
        }),
        serde_json::json!({
            "id": 3,
            "name": "Aretes de Filigrana",
            "artisan": "Isabel Chi",
            "price": 1200,
            "category": "joyeria",
            "img": "https://images.unsplash.com/photo-1611652032935-a6ce59b4c03d?q=80&w=1887&auto=format&fit=crop",
            "desc": "Elegantes aretes de plata trabajados con la delicada técnica de filigrana."
        })
    ];

    if let Some(producto) = productos.into_iter().find(|p| p["id"] == product_id) {
        HttpResponse::Ok().json(producto)
    } else {
        HttpResponse::NotFound().body("Producto no encontrado")
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "🚀 Servidor de Maya Digital funcionando correctamente",
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "database": "connected",
        "auth": "enabled",
        "bookings": "available",
        "businesses": "available",
        "version": "1.0.0"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();
    
    // Configurar logs
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Conectar a la base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10) // ✅ Aumentar conexiones para mejor rendimiento
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // ✅ Verificar conexión de BD
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => println!("✅ Base de datos PostgreSQL conectada correctamente"),
        Err(e) => {
            eprintln!("❌ Error conectando a la base de datos: {}", e);
            std::process::exit(1);
        }
    }

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("127.0.0.1:{}", port);

    println!("🚀 Servidor de Maya Digital iniciado en http://{}", bind_address);
    println!("📊 Base de datos PostgreSQL conectada");
    println!("🔐 Sistema de autenticación habilitado");
    println!("📋 Sistema de reservas disponible");
    println!("🏢 Sistema de registro de negocios disponible");

    HttpServer::new(move || {
        // ✅ CORS CORREGIDO - Más permisivo y robusto
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("https://localhost:3000") // Para HTTPS si lo usas
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                "Authorization", 
                "Content-Type", 
                "Accept",
                "Origin",
                "User-Agent",
                "X-Requested-With"
            ])
            .expose_headers(vec!["Content-Type", "Authorization"])
            .supports_credentials() // ✅ Permitir cookies si las usas
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            
            // ✅ Ruta de health check mejorada
            .service(health)
            .route("/api/health", web::get().to(|| async {
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "🚀 API de Maya Digital funcionando",
                    "status": "healthy"
                }))
            }))
            
            // Rutas de autenticación
            .route("/api/auth/register", web::post().to(register))
            .route("/api/auth/login", web::post().to(login))
            .route("/api/auth/me", web::get().to(me))
            
            // Rutas de administración - HOTELES (requieren autenticación)
            .route("/api/admin/hotels/pending", web::get().to(get_pending_hotels))
            .route("/api/admin/hotels/{id}/approve", web::put().to(approve_hotel))
            .route("/api/admin/hotels/{id}/reject", web::put().to(reject_hotel))
            .route("/api/admin/hotels", web::get().to(get_all_hotels))
            .route("/api/admin/stats", web::get().to(get_admin_stats))
            .route("/api/admin/metrics", web::get().to(get_admin_metrics))
            .route("/api/admin/businesses", web::get().to(get_admin_businesses))
            .route("/api/admin/bookings", web::get().to(get_admin_bookings))
            .route("/api/admin/search-analytics", web::get().to(get_search_analytics))
            
            // 🔥 NUEVAS RUTAS DE ADMINISTRACIÓN - NEGOCIOS (requieren autenticación)
            .route("/api/admin/businesses/pending", web::get().to(get_pending_businesses))
            .route("/api/admin/businesses/{id}/approve", web::put().to(approve_business))
            .route("/api/admin/businesses/{id}/reject", web::put().to(reject_business))

            // Rutas para dueños de hoteles (requieren autenticación)
            .route("/api/hotels", web::post().to(create_hotel))
            .route("/api/hotels/my-hotels", web::get().to(get_my_hotels))
            .route("/api/hotels/{id}", web::put().to(update_hotel))
            .route("/api/hotels/{id}", web::delete().to(delete_hotel))
            .route("/api/hotels/{id}", web::get().to(get_hotel_detail))
            .route("/api/portal/hotels/{hotel_id}/bookings", web::get().to(get_hotel_bookings))
            
            // 🔥 RUTAS PARA NEGOCIOS/RESTAURANTES (requieren autenticación)
            .route("/api/businesses", web::post().to(create_business))
            .route("/api/businesses/my-businesses", web::get().to(get_my_businesses))
            .route("/api/businesses/{id}", web::get().to(get_business_detail))
            .route("/api/businesses/{id}", web::put().to(update_business))
            .route("/api/businesses/{id}", web::delete().to(delete_business))
            
            // Rutas de reservas (requieren autenticación)
            .route("/api/bookings", web::post().to(create_booking))
            .route("/api/bookings/my-bookings", web::get().to(get_my_bookings))
            .route("/api/bookings/{id}/cancel", web::put().to(cancel_booking))

            // Rutas públicas existentes (compatibilidad)
            .route("/api/hoteles", web::get().to(get_all_approved_hotels))
            .service(get_restaurantes)
            .service(get_experiencias)
            .service(get_productos)
            .service(get_producto_by_id)
            
            // ✅ Ruta catch-all para debug (OPCIONAL)
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Endpoint no encontrado",
                    "available_endpoints": [
                        "POST /api/auth/register",
                        "POST /api/auth/login", 
                        "GET /api/auth/me",
                        "GET /health",
                        "GET /api/health"
                    ]
                }))
            }))
    })
    .bind(bind_address)?
    .run()
    .await
}