# Serial Vau - Tauri

## Migrations

Migrations are created automatically on application start as `home/.serial-vau/sqlite.db`

```bash
touch sqlite.db

export DATABASE_URL="sqlite:sqlite.db"
export MIGRATION_DIR="src-tauri/src/app/database/database_impl/sqlite_database_service/migration"

sea-orm-cli migrate --migration-dir $MIGRATION_DIR up

sea-orm-cli migrate --migration-dir $MIGRATION_DIR down
```

## Create Entities

```bash
export ENTITY_DIR="src-tauri/src/app/database/database_impl/sqlite_database_service/entity"

sea-orm-cli generate entity -o $ENTITY_DIR
```
