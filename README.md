
# User Management API (Rust + Actix-web)

## Objective

Extend the Stage 1 application by implementing user management with authentication, database integration, and full CRUD operations using Rust and Actix-web.

This project demonstrates secure backend development practices, API design, and documentation in a production-ready Rust environment.

---

## Project Overview

The application provides a RESTful API for managing users, including authentication, authorization, and secure data handling. It integrates a database for persistent storage and follows standard API response conventions.

---

## Features

* Full CRUD operations for users
* Authentication using JWT or session-based mechanism
* Secure password hashing
 
# User Management API

This repository contains a Rust-based User Management API implementing typical user lifecycle features: registration, email verification, password resets, authentication, roles & permissions, file uploads, audit logging, and more.

## Contents

- **Overview** — high-level description of the project
- **Getting started** — prerequisites and local setup
- **Database & migrations** — how to run migrations
- **Running the app** — start the server
- **API summary** — key endpoints and models
- **Data model UML** — visual diagram of the schema
- **Contributing** — how to help

## Overview

This API provides a foundation for user management in web applications: user accounts, profiles, email verification, password reset flows, session/refresh token handling, API keys for services, role/permission management, login history, audit logs, and file uploads for user assets.

The project is implemented in Rust and uses `sqlx` for database migrations and queries. The `migrations/` folder contains schema migrations (Postgres). See the `migrations/` directory for exact table definitions.

## Tech stack

- Rust
- sqlx (migrations & DB access)
- PostgreSQL

## Getting started

Prerequisites:

- Rust toolchain (rustc, cargo)
- PostgreSQL instance
- `sqlx` CLI (for migrations)

Quick setup (example):

```bash
# set DB URL (example)
export DATABASE_URL=postgres://user:pass@localhost:5432/user_management

# install sqlx-cli if needed
cargo install sqlx-cli --no-default-features --features postgres

# run DB migrations
sqlx migrate run

# run the server
cargo run
```

Adjust `DATABASE_URL` and other env vars as required by your environment or by `src/database_config.rs`.

## Database & migrations

All migrations live in the `migrations/` folder. To apply them locally:

```bash
sqlx migrate run
```

Migration files include the schema for:

- users
- email_verifications
- password_resets
- login_history
- profile
- roles, permissions, role_permission relations
- file_uploads
- intro_videos
- api_keys
- refresh_tokens
- audit_logs

## Running the app

Start the server with:

```bash
cargo run
```

Server configuration (ports, DB connection, etc.) is read from environment variables (see `src/database_config.rs`).

## API summary (high level)

The project exposes endpoints covering:

- User registration and login
- Email verification
- Password reset request / perform
- Profile retrieval and update
- Role and permission administration
- Uploading files (user avatars, assets)
- Intro video metadata storage
- API key issuance and management
- Refresh token management for sessions
- Audit logs recording important events

See `src/models.rs` for data structures and the `migrations/` SQL files for exact table definitions.

## Data model UML

The following diagram shows the main entities and relationships. View the image below or open `umldaigram.png` directly.

![Data Model UML](umldaigram.png)

If you want a PlantUML or alternative format, I can add that too.

## Contributing

Contributions are welcome. Typical workflows:

1. Fork the repo
2. Create a feature branch
3. Add tests where applicable
4. Submit a pull request explaining the change

Please keep changes focused and add migration scripts for any schema changes.

## License

This project does not include a license file. If you want a license added, tell me which one and I will add it.

---

Files updated/added in this change:

- `README.md` — updated with detailed instructions and UML reference


