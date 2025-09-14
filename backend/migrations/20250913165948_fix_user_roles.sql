-- Fix user roles constraint to include business_owner
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_check;
ALTER TABLE users ADD CONSTRAINT users_role_check 
    CHECK (role IN ('admin', 'hotel_owner', 'customer', 'business_owner'));

-- Verify the fix
SELECT 'User roles constraint updated successfully' AS status;
