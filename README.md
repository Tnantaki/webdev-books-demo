<h1 align="center">Book Store</h1>

<div align="center"><strong>Book store web dev</strong></div>

<div align="center">
  <h4>
    <a href="#config">
      Config
    </a>
    <span> | </span>
    <a href="#usage">
      Usage
    </a>
    <span> | </span>
    <a href="#api">
      API
    </a>
    <span> | </span>
    <a href="#stack">
      Stack
    </a>
  </h4>
</div>

## Config

- inside `backend` directory

```
cargo run -- create-admin
```

## Usage

- inside `backend` directory

```rust
cargo run
```

## Purchase Flow

1. User click "Add To Cart"
   └─> add book item to cart_items table
2. User clicks "Checkout"
   └─> Read items from cart_items table
3. Create Order
   └─> Validate book stock
   └─> INSERT into orders table, (Mark order as 'pending')
   └─> INSERT into order_items table (copy cart items with price snapshot)
   └─> CLEAR cart_items for those purchased items
4. User click "Pay"
   └─> Validate book stock
   └─> Update book stock
   └─> Mark order as 'paid'
   User click "Cancel"
   └─> Mark order as 'cancelled'
5. User can continue shopping
   └─> Cart is empty, ready for new items

## API

### Authentication

- [x] POST /api/auth/login # Login (returns JWT token in cookies)
- [x] POST /api/auth/logout # Logout
- [x] POST /api/auth/refresh # Refresh access token
- [ ] GET /api/auth/me # Get current user info

### User

- [x] POST /api/user/register # Register new user
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

### Query Parameters for GET /api/books

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
- [x] PUT /api/cart/items/:id # Update quantity
- [x] DELETE /api/cart/items/:id # Remove from cart
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

## Front end Hybrid rendering
🟢 SSR (Server Load Functions)
- GET `/api/books/*` SEO, initial load
- GET `/api/auth/me` Check login status
🔵 Client-Side (Direct to Axum)
- POST `/api/auth/login, logout, refresh` Form submission (mutation)
- GET, POST, PUT, DELETE `/api/user/*`  for admin
- POST, PUT, DELETE `/api/books` for admin to mutation data

<h2 align="center">Stack</h2>

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=postgres,rust,svelte,bun" />
  </a>
</p>
