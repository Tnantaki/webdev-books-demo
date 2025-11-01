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

## Front end Hybrid rendering

🟢 SSR (Server Load Functions)

- GET `/api/books/*` SEO, initial load
- GET `/api/auth/me` Check login status
  🔵 Client-Side (Direct to Axum)
- POST `/api/auth/login, logout, refresh` Form submission (mutation)
- GET, POST, PUT, DELETE `/api/user/*` for admin
- POST, PUT, DELETE `/api/books` for admin to mutation data

<h2 align="center">Stack</h2>

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=postgres,rust,svelte,bun" />
  </a>
</p>
