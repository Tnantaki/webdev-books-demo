<h1 align="center">Book Store</h1>

<div align="center"><strong>Book store web dev</strong></div>

<div align="center">
  <h4>
    <a href="#config">
      Config
    </a>
    <span> | </span>
    <a href="#stack">
      Stack
    </a>
    <span> | </span>
    <a href="#stack">
      Credit
    </a>
    <span> | </span>
    <a href="#purchase-flow">
      Purchase Flow
    </a>
  </h4>
</div>

## Config

1. create `.env` file in `frontend` directory and specify backend url
```bash
# ./frontend/.env
PUBLIC_API_BASE="http://localhost:3000/api"
```

2. run docker compose
```bash
# ./
podman compose up
```

3. [optional] We prepare mockup book data for you by running command.
```bash
podman exec backend_server ./book-store database seed
```

4. [optional] Running following command for create admin account.
```bash
podman exec -it backend_server ./book-store create-admin
```

5. access via url `http://localhost:4173`

<h2 align="center">Stack</h2>

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=postgres,rust,svelte,bun" />
  </a>
</p>

## Credit

- mockup data from `https://books.toscrape.com`

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

```text
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
