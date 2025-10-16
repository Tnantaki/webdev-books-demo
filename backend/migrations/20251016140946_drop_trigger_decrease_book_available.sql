-- Add migration script here

-- Drop old trigger and function
DROP TRIGGER IF EXISTS trigger_decrease_availability ON orders;
DROP FUNCTION IF EXISTS decrease_book_availability();