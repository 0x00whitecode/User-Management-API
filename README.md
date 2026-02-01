
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
* Database integration
* Input validation for all requests
* Pagination for list endpoints
* Environment variable management for secrets
* API documentation using OpenAPI/Swagger
* Deployed application

---

## Tech Stack

* **Language:** Rust
* **Web Framework:** Actix-web
* **Database:** PostgreSQL / MySQL / SQLite (via Diesel or SQLx)
* **Authentication:** JWT or session-based authentication
* **Password Hashing:** Argon2 or bcrypt
* **API Documentation:** OpenAPI / Swagger
* **Deployment:** Docker / Cloud provider (Render, Railway, AWS, etc.)

---

## Authentication & Security

* User passwords are hashed before storage using a secure hashing algorithm
* Authentication tokens are required for protected endpoints
* Secrets such as database credentials and JWT keys are stored in environment variables
* Input data is validated before processing

---

## Core API Endpoints

| Method | Endpoint      | Description               |
| ------ | ------------- | ------------------------- |
| POST   | `/users`      | Create a new user         |
| POST   | `/auth/login` | Authenticate user (login) |
| GET    | `/users`      | Fetch users (paginated)   |
| PUT    | `/users/{id}` | Update user information   |
| DELETE | `/users/{id}` | Delete a user             |

---

## Request Validation

* All incoming requests are validated using structured request models
* Invalid or missing fields return appropriate HTTP error responses

---

## Pagination

The users list endpoint supports pagination using query parameters:

* `page` – page number
* `limit` – number of records per page

This ensures efficient data retrieval and scalability.

---

## Standard Response Format

All API responses follow a consistent format:

```json
{
  "success": true,
  "data": {},
  "message": "Operation successful"
}
```

---

## API Documentation

* The API is documented using OpenAPI/Swagger
* Documentation includes:

  * Endpoint descriptions
  * Request and response schemas
  * Authentication requirements
* Swagger UI is available at:

```
/api-docs
```

---

## Environment Variables

Sensitive configuration values are managed using environment variables:

```env
DATABASE_URL=
JWT_SECRET=
SERVER_PORT=
```

---

## Deployment

* The application is deployed to a cloud environment
* Environment variables are configured securely in production
* The deployed API is accessible via a public URL

---

## Evaluation Criteria

* Correct implementation of CRUD operations
* Secure authentication and password handling
* Proper database usage
* Input validation and pagination
* Availability of API documentation

---

## Learning Outcomes

Participants will gain practical experience with:

* Rust backend development using Actix-web
* Database integration in Rust applications
* Authentication and API security
* Request validation and pagination
* API documentation and deployment

---

## License

This project is intended for educational and evaluation purposes.

---

