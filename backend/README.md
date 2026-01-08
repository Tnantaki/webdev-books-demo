# Reminer

## To run back-end in localhost
1. Prepare database server by running it in `podman`
```bash
# on path ./backend
podman run --name prosgres_db -p 5431:5432 -e POSTGRES_PASSWORD=123456 -d postgres:17 # create container

podman exec -it prosgres_db bash # exec container

psql -U postgres # connect to postgres

CREATE DATABASE book_store_db; # create database
```

2. if didn't have `sqlx` command, you must install it by run the command. on
```bash
# on path ./backend
cargo install sqlx-cli
```

3. run migration for create tables in databases server
```bash
sqlx migrate run --database-url postgres://postgres:123456@localhost:5432/book_store_db
```

4. [Optional] mockup book data
```bash
cargo run -- database seed
```

5. compile as dev mode and run
```bash
cargo run
```

## API
### Authentication
- [x] POST /api/auth/signup  # Register new user
- [x] POST /api/auth/login   # Login (returns JWT token in cookies)
- [x] POST /api/auth/logout  # Logout
- [x] POST /api/auth/refresh # Refresh access token
- [x] GET  /api/auth/me      # Get current user info

### User
- **Admin User Management**
- [x] GET /api/users # List all users (admin only)
- [x] GET /api/users/:id # Get user details (admin only)
- [ ] PUT /api/users/:id # Update user (admin only)
- [x] DELETE /api/users/:id # Delete user (admin only)

### Book
- [x] GET /api/books # List all books (with pagination, filters)
- [x] GET /api/books/:id # Get book details
- [ ] GET /api/books/search # Search books by title, author, etc.
- [ ] GET /api/books/categories # Get all categories
- [ ] GET /api/books/category/:name # Get books by category

- **Admin Book Management**
- [x] POST /api/books # Create book (admin only)
- [x] PUT /api/books/:id # Update book (admin only)
- [x] DELETE /api/books/:id # Delete book (admin only)

- **Query Parameters for GET /api/books/page**
- page - Page number (default: 1)
- limit - Items per page (default: 20)
- category - Filter by category
- author - Filter by author
- min_price - Minimum price
- max_price - Maximum price
- sort - Sort by (price, title, created_at, rating)
- order - asc or desc

### Cart Items
- [x] GET /api/cart # Get cart items
- [ ] DELETE /api/cart # Clear cart
- [x] POST /api/cart/item # Add to cart (DB)
- [x] PUT /api/cart/item/:id # Update quantity
- [x] DELETE /api/cart/item/:id # Remove from cart
- [x] POST /api/cart/checkout # Create order from cart
- [ ] POST /api/cart/save-for-later/:id # Move to saved items

### Orders

- [x] GET /api/orders # Get user's order history
- [x] GET /api/orders/:id # Get specific order details
- [x] PUT /api/orders/:id/pay # Pay order
- [x] PUT /api/orders/:id/cancel # Cancel order
- [x] GET /api/orders/item/:id # Get order item detail
- **Admin Order Management**
- [ ] GET /api/admin/orders # Get all orders (admin only)
- [ ] PUT /api/admin/orders/:id/status # Update order status (admin only)

### Ratings & Reviews

- [x] GET /api/books/:id/ratings # Get all ratings for a book
- [x] POST /api/books/:id/ratings # Add/update rating for a book
- [ ] DELETE /api/books/:id/ratings # Delete user's rating
- [ ] GET /api/users/me/ratings # Get current user's ratings

### Users (Profile Management)

- [ ] GET /api/users/me # Get current user profile
- [ ] PUT /api/users/me # Update profile
- [ ] PUT /api/users/me/password # Change password
- [ ] DELETE /api/users/me # Delete account

### Statistics (Optional)

- [ ] GET /api/admin/stats/sales # Sales statistics (admin only)
- [ ] GET /api/admin/stats/books # Book statistics (admin only)
- [ ] GET /api/books/:id/stats # Individual book statistics

## Database Migration Management
- Create a new migration
  ```bash
    sqlx migrate add <name_migration_file>
    # example:
    # sqlx migrate add add_updated_at_triggers
  ```
- Check migration status
  ```bash
    sqlx migrate info --database-url postgres://postgres:123456@localhost:5432/book_store_db
    # 20251011113427/installed init database
    # 20251011145956/pending add updated at triggers
  ```
- Run migration
  ```bash
    sqlx migrate run --database-url postgres://postgres:123456@localhost:5432/book_store_db
  ```

- Run the SQL directly in psql for inspect error sql script (sqlx migrate run didn't show error detail) 
  ```bash
    psql -h localhost -p 5432 -U postgres -d book_store_db -f migrations/20251015101213_add_order_and_rating_table.sql
  ```
  
- Reset Migration in database
  ```sql
    -- Connect to your database
    psql -h localhost -p 5432 -U postgres -d book_store_db
    
    -- Check the migrations table
    SELECT * FROM _sqlx_migrations;
    
    -- Delete the failed migration record
    DELETE FROM _sqlx_migrations WHERE version = 20251015101213;
  ```

## Database Posgres
- To create Schema from database (this will generate fresh schema from database schema)
  ```bash
  	pg_dump --schema-only your_database > current_schema.sql
   	# For example:
    pg_dump --schema-only postgres://postgres:123456@localhost:5432/book_store_db > current_schema.sql
  ```