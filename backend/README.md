# Reminer

### Database Migration Management
- Create a new migration
  ```bash
    sqlx migrate add <name_migration_file>
    # example:
    # sqlx migrate add add_updated_at_triggers
  ```
  'sqlx migrate add add_updated_at_triggers'
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
