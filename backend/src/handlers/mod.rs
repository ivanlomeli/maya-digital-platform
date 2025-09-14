// handlers/mod.rs - VERSIÓN COMPLETA CON FUNCIÓN 'me'

pub mod auth;
pub mod hotel;
pub mod booking;
pub mod admin;
pub mod business;

// Re-export all handlers from auth
pub use auth::{register, login, me}; // ✅ Exportar explícitamente 'me'

// Re-export all handlers from hotel
pub use hotel::*;

// Re-export all handlers from booking
pub use booking::*;

// Re-export business handlers
pub use business::{
    create_business, 
    get_my_businesses, 
    get_business_detail, 
    update_business, 
    delete_business
};

// Re-export admin handlers
pub use admin::{
    get_admin_stats,
    get_admin_metrics,
    get_admin_businesses,
    get_admin_bookings,
    get_search_analytics,
    get_pending_hotels,
    approve_hotel,
    reject_hotel,
    get_all_hotels,
    // Nuevos handlers para negocios
    get_pending_businesses,
    approve_business,
    reject_business
};