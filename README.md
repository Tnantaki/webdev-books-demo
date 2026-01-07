<h1 align="center">Book Store</h1>

<div align="center"><strong>Book store web dev</strong></div>

<div align="center">
  <h4>
  <a href="#prepare-database">
    Prepare Database
  </a>
  <span> | </span>
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

## Prepare Database

- create database server

```bash
podman run --name prosgres_db -p 5431:5432 -e POSTGRES_PASSWORD=123456 -d postgres:17 # create container

podman exec -it prosgres_db bash # exec container

psql -U postgres # connect to postgres

CREATE DATABASE book_store_db; # create database
```

- if can't run `sqlx` command, you must install it first.

```bash
cargo install sqlx-cli
```

- run migration

```bash
sqlx migrate run --database-url postgres://postgres:123456@localhost:5432/book_store_db
```

## Config

- inside `backend` directory

```bash
cargo run -- create-admin
```

## Usage

- inside `backend` directory

```bash
cargo run
```

- inside `frontend` directory

```bash
bun run
```

## Purchase Flow
```
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
```   
## Visual Flow
```
┌─────────┐         ┌─────────────┐          ┌──────────────┐
│ Browser │         │  SvelteKit  │          │ Rust Backend │
└────┬────┘         └──────┬──────┘          └──────┬───────┘
     │                     │                        │
     │ GET /cart           │                        │
     ├────────────────────>│                        │
     │  (with cookie)      │                        │
     │            ┌────────▼────────┐               │
     │            │Check token exist│               │
     │            └────────┬────────┘               │
     │                     │                        │
     │                  No token?                   │
     │              redirect to /login              │
     │                     │                        │
     │                  Has token?                  │
     │                     │ GET /api/cart          │
     │                     │ + Authorization header │
     │                     ├───────────────────────>│
     │                     │                ┌───────▼───────┐
     │                     │                │ VERIFY Token  │
     │                     │                │(REAL SECURITY)│
     │                     │                └───────┬───────┘
     │                     │      Valid: cart data  │
     │                     │<───────────────────────┤
     │               ┌─────▼─────┐                  │
     │               │ Combine   │                  │
     │               │ with HTML │                  │
     │               └─────┬─────┘                  │
     │   Rendered page     │                        │
     │<────────────────────┤                        │
     │                     │                        │
```

<h2 align="center">Stack</h2>

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=postgres,rust,svelte,bun" />
  </a>
</p>
