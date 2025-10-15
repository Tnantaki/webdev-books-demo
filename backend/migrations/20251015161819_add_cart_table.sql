-- Add migration script here

BEGIN;

-- Cart items (temporary, can be modified)
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, book_id) -- One entry per book per user
);

-- Trigger to update cart_items timestamp
CREATE TRIGGER update_cart_items_updated_at 
BEFORE UPDATE ON cart_items
FOR EACH ROW 
EXECUTE FUNCTION update_updated_at_column();

COMMIT;