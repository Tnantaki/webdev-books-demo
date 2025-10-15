# Reminer

### Database Migration Management
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
- Check migration status
  ```bash
    sqlx migrate run --database-url postgres://postgres:123456@localhost:5432/book_store_db
  ```
  
  psql -h localhost -p 5432 -U postgres -d book_store_db

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
