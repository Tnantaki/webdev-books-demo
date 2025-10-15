-- Add migration script here

-- change column name
ALTER TABLE books
RENAME COLUMN img_path TO image_id;

-- change datatype (requires USING clause for VARCHAR -> UUID conversion)
ALTER TABLE books
ALTER COLUMN image_id TYPE UUID USING image_id::UUID;

-- add foreign key constraint
ALTER TABLE books 
ADD CONSTRAINT fk_books_image 
FOREIGN KEY (image_id) 
REFERENCES images(id) 
ON DELETE SET NULL;