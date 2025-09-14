// models/business.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Business {
    pub id: i32,
    pub owner_id: i32,
    pub business_type: String,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub status: String, // Cambiar a String
    pub business_data: serde_json::Value,
    pub operating_hours: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "business_status", rename_all = "lowercase")]
pub enum BusinessStatus {
    Pending,
    Approved,
    Rejected,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessImage {
    pub id: i32,
    pub business_id: i32,
    pub image_url: String,
    pub image_type: String,
    pub display_order: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBusinessRequest {
    pub business_type: String,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub business_data: BusinessData,
    pub operating_hours: HashMap<String, OperatingHours>,
    pub images: Option<Vec<BusinessImageInput>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessData {
    // Para restaurantes
    pub specialty: Option<String>,
    pub cuisine_type: Option<String>,
    pub price_range: Option<String>,
    pub capacity: Option<i32>,
    pub delivery_available: Option<bool>,
    pub reservation_required: Option<bool>,
    pub services: Option<Vec<String>>,
    pub menu_highlights: Option<Vec<MenuHighlight>>,
    
    // Para hoteles
    pub hotel_type: Option<String>,
    pub star_rating: Option<i32>,
    pub total_rooms: Option<i32>,
    pub amenities: Option<Vec<String>>,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub room_types: Option<Vec<RoomType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuHighlight {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<String>,
    pub image_url: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomType {
    pub name: String,
    pub description: Option<String>,
    pub price_per_night: Option<f64>,
    pub max_occupancy: Option<i32>,
    pub amenities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatingHours {
    pub open: String,
    pub close: String,
    pub closed: bool,
}

#[derive(Debug, Deserialize)]
pub struct BusinessImageInput {
    pub image_url: String,
    pub image_type: String,
    pub display_order: i32,
}

#[derive(Debug, Serialize)]
pub struct BusinessResponse {
    pub id: i32,
    pub business_type: String,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub status: String, // Cambiar a String
    pub business_data: serde_json::Value,
    pub operating_hours: serde_json::Value,
    pub images: Vec<BusinessImage>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Business> for BusinessResponse {
    fn from(business: Business) -> Self {
        BusinessResponse {
            id: business.id,
            business_type: business.business_type,
            name: business.name,
            description: business.description,
            location: business.location,
            address: business.address,
            phone: business.phone,
            email: business.email,
            website: business.website,
            status: business.status,
            business_data: business.business_data,
            operating_hours: business.operating_hours,
            images: vec![], // Se cargarán por separado
            created_at: business.created_at,
        }
    }
}
// Agregar al final del archivo
impl Business {
    pub fn validate_status(&self) -> Result<(), String> {
        match self.status.as_str() {
            "pending" | "approved" | "rejected" | "suspended" => Ok(()),
            _ => Err(format!("Invalid business status: {}", self.status))
        }
    }
    
    pub fn is_editable(&self) -> bool {
        matches!(self.status.as_str(), "pending" | "rejected")
    }
}
