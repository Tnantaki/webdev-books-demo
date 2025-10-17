-- Add migration script here
ALTER TABLE books
ADD COLUMN average_rating FLOAT NOT NULL DEFAULT 0 CHECK (average_rating >= 0);

ALTER TABLE books
ADD COLUMN total_ratings INTEGER NOT NULL DEFAULT 0 CHECK (total_ratings >= 0);

-- This function calculates and updates average_rating and total_ratings
CREATE OR REPLACE FUNCTION update_book_rating_stats()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE books
    SET
        average_rating = (
            SELECT AVG(rating)::FLOAT
            FROM ratings
            WHERE book_id = OLD.book_id
        ),
        total_ratings = (
            SELECT COUNT(*)::INTEGER
            FROM ratings
            WHERE book_id = OLD.book_id
        )
    WHERE id = OLD.book_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers to update rating in books table when rating get update
CREATE TRIGGER trigger_rating_changes
AFTER INSERT OR UPDATE OR DELETE ON ratings
FOR EACH ROW
EXECUTE FUNCTION update_book_rating_stats();
