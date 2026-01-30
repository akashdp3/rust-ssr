# rust-ssr

A learning project for building a React Server Components (RSC) framework from scratch in Rust.

## Current Progress

| Phase | Milestone | Status |
|-------|-----------|--------|
| 1 | Basic HTTP Server | Done |
| 1 | Embed JavaScript Runtime | Done |
| 1 | ES Module Loading | Not Started |
| 2 | Server-Side React Rendering | Not Started |
| 2 | File-Based Routing | Not Started |
| 2 | Client Hydration | Not Started |

See [TODO.md](./TODO.md) for the full roadmap.

## Features

- Async HTTP server running on `127.0.0.1:3000`
- Static file serving with path traversal protection
- Basic routing (`/`, `/about`, `/name/{name}`)
- JavaScript execution via embedded Boa engine (`POST /js`)

## Getting Started

### Prerequisites

- Rust toolchain (rustup recommended)

### Installation

```bash
git clone <repo-url>
cd rust-ssr
cargo build
```

## Usage

### Run the server

```bash
cargo run
```

The server starts at `http://127.0.0.1:3000`

## Project Structure

```
rust-ssr/
├── src/
│   ├── main.rs       # Entry point and router setup
│   ├── handlers.rs   # Request handlers
│   └── tests.rs      # Integration tests
├── static/           # Static files served at /static/*
├── docs/             # Documentation
├── TODO.md           # Learning roadmap with milestones
└── Cargo.toml        # Dependencies
```

## Tech Stack

- **Axum** - Web framework
- **Tokio** - Async runtime
- **Boa Engine** - JavaScript runtime
- **Serde** - Serialization
