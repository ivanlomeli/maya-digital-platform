-- Archivo de Inicialización (Seed) para la Base de Datos de Maya Digital
TRUNCATE TABLE users, hotels, restaurants, menu_items, b2b_services, b2b_leads RESTART IDENTITY CASCADE;

INSERT INTO users (id, name, email, password_hash, user_type) VALUES
(1, 'Admin Maya Digital', 'admin@mayadigital.com', 'hash_falso_admin', 'admin'),
(2, 'Carlos Puc', 'carlos.puc@hotelbalam.com', 'passwordsegura', 'owner'),
(3, 'Ana Chi', 'ana.chi@lacoctelera.com', 'passwordsegura', 'owner')
ON CONFLICT (id) DO NOTHING;

INSERT INTO hotels (id, owner_id, name, location, image_url, price_per_night, status) VALUES
(1, 2, 'Hotel Balam Kú', 'Tulum, Quintana Roo', '/uploads/hotel_tulum.jpg', 2850, 'Approved'),
(2, 3, 'Hacienda Kaan', 'Valladolid, Yucatán', '/uploads/hotel_hacienda.jpg', 4200, 'Approved'),
(3, 2, 'Cabañas Zazil-Ha', 'Isla Mujeres, Quintana Roo', '/uploads/hotel_cabanas.jpg', 1900, 'Pending')
ON CONFLICT (id) DO NOTHING;

INSERT INTO restaurants (id, owner_id, name, location, specialty, image_url, status) VALUES
(1, 3, 'La Coctelera del Caribe', 'Playa del Carmen', 'Mariscos y Pescado Fresco', '/uploads/rest_mariscos.jpg', 'Approved'),
(2, 2, 'El Rincón del Maíz', 'Mérida, Yucatán', 'Cocina Yucateca Tradicional', '/uploads/rest_yucateco.jpg', 'Pending')
ON CONFLICT (id) DO NOTHING;

INSERT INTO menu_items (restaurant_id, name, description, price, category) VALUES
(1, 'Ceviche Mixto', 'Pescado fresco, camarón y pulpo marinado en cítricos.', 220.00, 'Entradas'),
(1, 'Tacos de Pescado estilo Baja', 'Tres tacos de pescado rebozado con col y salsa de chipotle.', 180.00, 'Plato Fuerte'),
(2, 'Sopa de Lima', 'Caldo de pavo ligero con un toque de lima y tiras de tortilla frita.', 110.00, 'Entradas'),
(2, 'Cochinita Pibil', 'Carne de cerdo marinada en achiote, cocida lentamente.', 250.00, 'Plato Fuerte');

INSERT INTO b2b_services (id, owner_id, title, description, category) VALUES
(1, 2, 'Servicio de Lavandería Industrial para Hoteles', 'Ofrecemos lavado, secado y planchado de blancos para hoteles en la Riviera Maya.', 'Suministros'),
(2, 3, 'Tours de Buceo para Huéspedes (Comisión)', 'Asóciate con nosotros y ofrece tours de buceo a tus huéspedes. Ofrecemos 15% de comisión.', 'Tours y Comisiones')
ON CONFLICT (id) DO NOTHING;

\echo '✅ Base de datos restaurada con datos de prueba.'