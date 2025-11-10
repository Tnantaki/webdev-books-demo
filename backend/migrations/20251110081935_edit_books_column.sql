-- Add migration script here
BEGIN;

ALTER TABLE books
RENAME COLUMN price_in_pound TO price;

-- Create enum type for order status
CREATE TYPE order_status AS ENUM ('pending', 'paid', 'completed', 'cancelled');

ALTER TABLE orders
ALTER COLUMN order_status SET DEFAULT 'pending'::order_status;

ALTER TABLE orders
ALTER COLUMN order_status TYPE order_status
USING (order_status::order_status); -- for convert old's VARCHAR value to enum type

COMMIT;