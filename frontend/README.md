
## Front end Hybrid rendering

🟢 SSR (Server Load Functions)
- GET `/api/books/*` SEO, initial load
- GET `/api/auth/me` Check login status

🔵 Client-Side (Direct to Axum)
- POST `/api/auth/login, logout, refresh` Form submission (mutation)
- GET, POST, PUT, DELETE `/api/user/*` for admin
- POST, PUT, DELETE `/api/books` for admin to mutation data
