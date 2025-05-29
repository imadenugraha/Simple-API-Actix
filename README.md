# Actix Web Service Example

A simple REST API service built with Actix Web framework in Rust. This service provides basic endpoints for health checking and user greeting functionality.

## Features

- RESTful API endpoints
- JSON responses
- Path parameter handling
- Health check endpoint

## Prerequisites

- Rust 1.87.0 or later
- Cargo (Rust's package manager)

## Dependencies

- actix-web (4.11.0)
- serde (1.0.219)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/imadenugraha/Simple-API-Actix.git
cd Simple-API-Actix
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8000`

## API Endpoints

### 1. Root Endpoint
- **URL:** `/`
- **Method:** `GET`
- **Response Example:**
```json
{ "msg": "Hello from service A" }
```

### 2. Name Greeting
- **URL:** `/api/v1/{name}`
- **Method:** `GET`
- **Parameters:**
    - `name` (path parameter): Name to be greeted
- **Response Example:**
```json
{ "name": "Hello john", "service": "A" }
```

### 3. Health Check
- **URL:** `/up`
- **Method:** `GET`
- **Response Example:**
```json
{ "msg": "OK" }
```

## License

MIT
