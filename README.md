# Identity & Access Management (IAM) Platform

A production-inspired backend application built with **Rust**, **Axum**, **PostgreSQL**, and **SQLx** that provides secure authentication, authorization, organization management, Role-Based Access Control (RBAC), session management, API key management, and audit logging.

## Project Overview

The IAM Platform is designed to simulate an enterprise-grade authentication and authorization system similar to platforms like Auth0, Keycloak, Clerk, and AWS IAM.

The system answers three important questions:

- Who is the user?
- Which organization does the user belong to?
- What is the user allowed to do?

This project demonstrates secure backend architecture, REST API design, JWT authentication, RBAC implementation, PostgreSQL database design, and layered software architecture.

---

# Features

## Authentication

- User Registration
- User Login
- User Logout
- JWT Access Tokens
- Refresh Tokens
- Password Hashing
- Protected Routes

## User Management

- View Profile
- Update Profile

## Organization Management

- Create Organization
- Update Organization
- View Organizations

## Role-Based Access Control (RBAC)

- Create Roles
- Update Roles
- Delete Roles
- Assign Permissions
- Role-Permission Mapping
- Membership Management

## Permissions

- Create Permissions
- View Permissions
- Permission Evaluation

## Session Management

- View Active Sessions
- Revoke Sessions

## API Keys

- Generate API Keys
- View API Keys
- Delete API Keys
- Hashed API Key Storage

## Audit Logging

- User Activity Logging
- Organization Events
- Role Events
- API Key Events

## API Documentation

- Swagger UI Integration

---

# Tech Stack

| Technology | Purpose |
|------------|---------|
| Rust | Programming Language |
| Axum | Web Framework |
| Tokio | Async Runtime |
| PostgreSQL | Database |
| SQLx | Database ORM/Driver |
| Serde | Serialization & Deserialization |
| UUID | Unique Identifier Generation |
| Chrono | Date & Time Handling |
| JsonWebToken | JWT Authentication |
| Validator | Request Validation |
| Tracing | Structured Logging |
| Utoipa | Swagger Documentation |

---

# Project Architecture

```
Client
   │
   ▼
Routes
   │
   ▼
Handlers
   │
   ▼
Services
   │
   ▼
Repositories
   │
   ▼
PostgreSQL Database
```

The project follows a layered architecture to separate responsibilities and improve maintainability.

---

# Project Structure

```
src/
│
├── app_state.rs
├── main.rs
│
├── config/
├── handlers/
├── middleware/
├── models/
├── repositories/
├── routes/
├── schemas/
├── services/
├── utils/
├── errors/
├── swagger/
│
migrations/
│
Cargo.toml
README.md
```

---

# Authentication Flow

```
Client

↓

POST /auth/login

↓

Validate Credentials

↓

Verify Password

↓

Generate JWT

↓

Create Session

↓

Return Access Token

↓

Protected APIs

↓

JWT Middleware

↓

Authorized Response
```

---

# Role Based Access Control (RBAC)

```
User

↓

Organization Membership

↓

Role

↓

Permissions

↓

Authorization Decision
```

Example Permissions

```
user:create
user:update
user:delete

organization:update

role:create

permission:create

api_key:create
```

---

# Database Design

The system consists of relational entities including:

- Users
- Organizations
- Memberships
- Roles
- Permissions
- Role Permissions
- Sessions
- API Keys
- Audit Logs

Relationships are implemented using foreign keys and UUID-based primary keys.

---

# REST API

## Authentication

| Method | Endpoint |
|---------|----------|
| POST | /auth/register |
| POST | /auth/login |
| POST | /auth/refresh |
| POST | /auth/logout |

---

## Users

| Method | Endpoint |
|---------|----------|
| GET | /users/me |
| PATCH | /users/me |

---

## Organizations

| Method | Endpoint |
|---------|----------|
| POST | /organizations |
| GET | /organizations |
| GET | /organizations/{id} |
| PATCH | /organizations/{id} |

---

## Roles

| Method | Endpoint |
|---------|----------|
| POST | /roles |
| GET | /roles |
| PATCH | /roles/{id} |
| DELETE | /roles/{id} |

---

## Permissions

| Method | Endpoint |
|---------|----------|
| POST | /permissions |
| GET | /permissions |

---

## Sessions

| Method | Endpoint |
|---------|----------|
| GET | /sessions |
| DELETE | /sessions/{id} |

---

## API Keys

| Method | Endpoint |
|---------|----------|
| POST | /api-keys |
| GET | /api-keys |
| DELETE | /api-keys/{id} |

---

# Installation

## Clone Repository

```bash
git clone https://github.com/Ankit686332/iam-platform.git

cd iam-platform
```

---

## Create Environment File

Create a `.env` file in the project root.

Example:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/iam_platform

JWT_SECRET=your_secret_key

HOST=127.0.0.1

PORT=3000
```

---

## Install Dependencies

```bash
cargo build
```

---

## Run Database Migrations

```bash
sqlx migrate run
```

---

## Run the Application

```bash
cargo run
```

---

# Swagger Documentation

Once the application starts, open:

```
http://localhost:3000/swagger-ui
```

---

# Health Check

```
GET /health
```

Expected Response

```
Database Connected
```

---

# Testing

Run all tests

```bash
cargo test
```

---

# Security Features

- Password Hashing
- JWT Authentication
- Refresh Tokens
- Role-Based Authorization
- API Key Hashing
- Request Validation
- Environment Variable Configuration
- Structured Logging
- SQL Injection Protection using SQLx
- Connection Pooling

---

# Software Engineering Principles

The project follows:

- Layered Architecture
- Separation of Concerns
- Single Responsibility Principle
- Repository Pattern
- Service Pattern
- RESTful API Design
- Dependency Injection using AppState
- Modular Code Organization

---

# Future Improvements

- Redis Caching
- Rate Limiting
- Email Verification
- Password Reset
- CI/CD Pipeline
- Docker Compose
- Kubernetes Deployment
- OAuth Login
- Multi-Factor Authentication (MFA)

---

# Author

**Ankit Mohanty**

Backend Major Project

Rust • Axum • PostgreSQL • SQLx

---

# License

This project is developed for educational purposes as part of a Backend Engineering major project.
