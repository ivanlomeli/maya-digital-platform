#!/bin/bash
echo "🔧 Verificando fixes aplicados..."

# Verificar compilación
echo "1. Verificando compilación Rust..."
cd backend
cargo check 2>&1 | head -10

echo "2. Verificando clippy..."
cargo clippy 2>&1 | head -5

echo "3. Verificando BD (requiere psql)..."
# Descomenta si tienes acceso directo a la BD:
# psql -d tu_database -c "SELECT role, COUNT(*) FROM users GROUP BY role;"

echo "✅ Verificación completada"
