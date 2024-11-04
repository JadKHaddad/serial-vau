# Serial Vau

*Welcome to **Serial Vau*** A Monitoring Tool for sending and reaciving data.

## Setup

### Requirements

- Windows system
- Rust
- tauri-cli
- Node / npm

## 💿 Commands

| Package Manager                                               | Command        |
|---------------------------------------------------------------|----------------|
| [ui & serial vau](https://tauri.app/)                         | `cargo tauri`  |
| [ui dev](https://docs.npmjs.com/cli/v7/commands/npm-install)  | `npm run dev`  |
| [serial vau](https://www.rust-lang.org/tools/install)         | `cargo run`    |

## ✨ Features

- 🛠️ **Subscription**: Activate Subscription to Subscribe to multiple devices.

- *More Features soon...*

### Screenshots

![alt text](screenshots/image.png)

## 💪 Support Serial Vau Development

- *Do you want to Contribut to Serial Vau?* **Just do it** ⚡

## Migrate

```bash
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
