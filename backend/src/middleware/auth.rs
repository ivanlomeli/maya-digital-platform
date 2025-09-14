use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures_util::future::{ok, err, Ready};

use crate::models::{UserInfo, UserRole};
use crate::utils::verify_jwt;

impl FromRequest for UserInfo {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Remove "Bearer " prefix
                    
                    // Use your existing verify_jwt function
                    match verify_jwt(token) {
                        Ok(claims) => {
                            // Convert role string to UserRole - FIXED
                            let role = match claims.role.as_str() {
                                "admin" => UserRole::Admin,
                                "hotel_owner" => UserRole::HotelOwner,
                                "business_owner" => UserRole::BusinessOwner,
                                "customer" => UserRole::Customer,
                                _ => UserRole::Customer,
                            };
                            
                            // Parse user ID from claims
                            let user_id = match claims.sub.parse::<i32>() {
                                Ok(id) => id,
                                Err(e) => {
                                    eprintln!("Error parsing user ID from token: {}", e);
                                    return err(ErrorUnauthorized("Invalid user ID in token"));
                                }
                            };
                            
                            // TODO: Implementar consulta a BD para obtener datos completos
                            // Por ahora, usar los datos básicos del token
                            let user_info = UserInfo {
                                id: user_id,
                                email: claims.email,
                                role,
                                first_name: "Usuario".to_string(), // Valor temporal hasta implementar consulta
                                last_name: "".to_string(),        // Valor temporal hasta implementar consulta
                                phone: None,
                            };
                            
                            return ok(user_info);
                        }
                        Err(e) => {
                            eprintln!("JWT verification failed: {}", e);
                            return err(ErrorUnauthorized("Invalid JWT token"));
                        }
                    }
                }
            }
        }
        
        err(ErrorUnauthorized("Invalid or missing authorization token"))
    }
}

pub fn require_role(required_role: &str) -> impl Fn(&UserInfo) -> bool + '_ {
    move |user: &UserInfo| {
        match (&user.role, required_role) {
            (UserRole::Admin, _) => true, // Admin can access everything
            (UserRole::HotelOwner, "hotel_owner") => true,
            (UserRole::BusinessOwner, "business_owner") => true,
            (UserRole::Customer, "customer") => true,
            _ => false,
        }
    }
}
