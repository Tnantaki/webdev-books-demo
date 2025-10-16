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
    └─> INSERT into orders table, (Mark order as 'pending')
    └─> INSERT into order_items table (copy cart items with price snapshot)
    └─> CLEAR cart_items for those purchased items
4. User click "Pay"
    └─> Mark order as 'paid'
   User click "Cancel"
    └─> Mark order as 'cancelled'
5. User can continue shopping
    └─> Cart is empty, ready for new items

## API

### Authentication
- [x] POST /auth/login   # Login (returns JWT token in cookies)
- [x] POST /auth/logout  # Logout
- [x] POST /auth/refresh # Refresh access token
- [ ] GET  /auth/me      # Get current user info

### User
- [x] POST /user/register # Register new user

### Admin User Management
- [x] GET /users # List all users (admin only)
- [x] GET /users/:id # Get user details (admin only)
- [ ] PUT /users/:id # Update user (admin only)
- [x] DELETE /users/:id # Delete user (admin only)

### Book
- [x] GET /books # List all books (with pagination, filters)
- [x] GET /books/:id # Get book details
- [x] POST /books # Create book (admin only)
- [x] PUT /books/:id # Update book (admin only)
- [x] DELETE /books/:id # Delete book (admin only)
- [ ] GET /books/search # Search books by title, author, etc.
- [ ] GET /books/categories # Get all categories
- [ ] GET /books/category/:name # Get books by category

### Query Parameters for GET /books

- page - Page number (default: 1)
- limit - Items per page (default: 20)
- category - Filter by category
- author - Filter by author
- min_price - Minimum price
- max_price - Maximum price
- sort - Sort by (price, title, created_at, rating)
- order - asc or desc

### Cart Items
- [x] POST   /api/cart/item               # Add to cart (DB)
- [x] GET    /api/cart                    # Get cart items
- [x] PUT    /api/cart/items/:id          # Update quantity
- [x] DELETE /api/cart/items/:id          # Remove from cart
- [ ] POST   /api/cart/checkout           # Create order from cart
- [ ] POST   /api/cart/save-for-later/:id # Move to saved items

### Orders

- [ ] GET /orders # Get user's order history
- [ ] GET /orders/:id # Get specific order details
- [ ] POST /orders # Create new order (checkout)
- [ ] PUT /orders/:id/cancel # Cancel order
- [ ] GET /orders/:id/items # Get order items

Admin Order Management
- [ ] GET /admin/orders # Get all orders (admin only)
- [ ] PUT /admin/orders/:id/status # Update order status (admin only)

### Cart (Optional - can be client-side only)

- [ ] GET /cart # Get user's cart
- [ ] POST /cart/items # Add item to cart
- [ ] PUT /cart/items/:id # Update cart item quantity
- [ ] DELETE /cart/items/:id # Remove item from cart
- [ ] DELETE /cart # Clear cart

### Ratings & Reviews

- [ ] GET /books/:id/ratings # Get all ratings for a book
- [ ] POST /books/:id/ratings # Add/update rating for a book
- [ ] DELETE /books/:id/ratings # Delete user's rating
- [ ] GET /users/me/ratings # Get current user's ratings

### Users (Profile Management)

- [ ] GET /users/me # Get current user profile
- [ ] PUT /users/me # Update profile
- [ ] PUT /users/me/password # Change password
- [ ] DELETE /users/me # Delete account

### Statistics (Optional)

- [ ] GET /admin/stats/sales # Sales statistics (admin only)
- [ ] GET /admin/stats/books # Book statistics (admin only)
- [ ] GET /books/:id/stats # Individual book statistics

<h4 align="center">Stack</h4>

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=postgres,rust" />
  </a>
</p>