# nhl-api

A personal project for tracking NHL statistics. Users are assigned teams per season, and game matchups with scores are recorded. Reference data (teams, game types) is seeded from the official NHL Stats API.

---

## Architecture

```
NHL Stats API (statsapi.web.nhl.com/api/v1)
    │
    ├── Rust CLI (api/)        ── writes ──► MySQL (mysql/)
    └── PHP Symfony command    ── writes ──► MySQL
                │
            Symfony app (app/)   [HTTP routes not yet implemented]
                │
             Nginx (nginx/)
                │
            Browser / client
```

Two separate implementations can fetch data from the NHL API into the shared MySQL database:
- a **Rust CLI** (`api/`) — fetches teams and game types
- a **Symfony console command** (`app/`) — fetches teams only

The Symfony web application layer exists but has no HTTP routes implemented yet.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Web framework | Symfony 4.3 (PHP 7.1.3+) |
| PHP ORM | Doctrine 2 |
| PHP HTTP client | Guzzle |
| Templates | Twig |
| Rust ORM | Diesel 1.4 (MySQL) |
| Rust HTTP client | reqwest |
| Serialization | serde / serde_json |
| Database | MySQL 8.0 |
| Reverse proxy | Nginx 1.15 |
| Containerization | Docker / Docker Compose |

---

## Directory Structure

```
nhl-api/
├── app/                    # Symfony PHP web application
│   ├── src/
│   │   ├── Command/        # CLI: FetchTeamsCommand (app:fetch:teams)
│   │   ├── Controller/     # HTTP controllers (empty — not yet implemented)
│   │   ├── Entity/         # Doctrine entity: Team
│   │   ├── Migrations/     # Doctrine migration: creates teams table
│   │   ├── Repository/     # TeamRepository
│   │   └── Service/        # Fetcher — calls NHL API, persists via Doctrine
│   ├── config/             # Symfony bundle / service / routing config
│   ├── public/index.php    # Front controller
│   └── Dockerfile
│
├── api/                    # Rust data-fetcher CLI
│   ├── src/
│   │   ├── bin/main.rs     # Entry point — fetches teams + game types
│   │   ├── lib.rs          # fetch_teams() and fetch_game_types()
│   │   ├── db.rs           # Database connection helper
│   │   ├── schema.rs       # Diesel schema definitions
│   │   └── model/          # Rust structs (API responses + DB models)
│   ├── migrations/         # Diesel migration: full schema
│   ├── Cargo.toml
│   └── Dockerfile
│
├── mysql/
│   ├── init/init.sql       # Full schema + seed data (one user pre-populated)
│   ├── .env.dist
│   └── Dockerfile
│
├── nginx/
│   ├── nhl.conf            # Reverse proxy: port 80 → PHP-FPM on app:9000
│   └── Dockerfile
│
├── docker-compose.yml
└── docker-compose.override.yml
```

---

## Database Schema

| Table | Description |
|---|---|
| `users` | User accounts (one pre-seeded: `natelenart`) |
| `roles` | `ROLE_ADMIN`, `ROLE_USER` |
| `user_roles` | Many-to-many: users ↔ roles |
| `teams` | NHL teams synced from API |
| `seasons` | Season identifiers (e.g. `20192020`) |
| `game_types` | Game classifications: PR (preseason), R (regular), P (playoffs), F (finals) |
| `user_teams` | Many-to-many: users ↔ teams per season |
| `matchups` | Scheduled games with optional scores (home/away teams, game type, datetime) |

Migrations exist in two places:
- `api/migrations/` — Diesel, covers the full schema
- `app/src/Migrations/` — Doctrine, covers `teams` only
- `mysql/init/init.sql` — runtime seed script, full schema + data

---

## Getting Started

```bash
# Start all services (MySQL, PHP-FPM, Nginx)
docker-compose up

# Seed teams via PHP
docker-compose exec app bin/console app:fetch:teams

# Seed teams + game types via Rust CLI (run manually — not in compose by default)
cd api && cargo run
```

The web app is accessible at `http://localhost/` (or the port mapped in `docker-compose.override.yml`).

---

## Known Issues & Deprecations

1. **Rust service not wired into Docker Compose** — the `api` service is commented out in `docker-compose.yml`; the Rust CLI must be run manually.

2. **Abandoned web server experiments in `api/src/lib.rs`** — commented-out Actix-web and raw TCP socket implementations (~lines 72–122) from an earlier attempt to expose the Rust service over HTTP.

3. **Deprecated Diesel codegen** — Diesel 1.4 uses the older `diesel_codegen` approach. Modern Diesel versions use proc-macro derives (`#[derive(Queryable, Insertable)]` without a separate codegen crate).

4. **MD5 password hashing** — `mysql/init/init.sql` hashes passwords as `MD5(CONCAT(salt, password))`. This is insecure; bcrypt or Argon2 should be used instead.

5. **Panic-prone Rust error handling** — `api/src/lib.rs` uses `.expect()` throughout with no graceful recovery on network or database errors.

6. **No HTTP routes in Symfony** — `app/src/Controller/` is empty. The web layer is a shell with no endpoints exposed.

7. **No tests** — `app/tests/` contains only a `.gitignore`.

8. **Duplicate data-fetching paths** — teams can be fetched by either the Rust CLI or the PHP command with no coordination or deduplication between the two.

---

## Development Status

| Feature | Status |
|---|---|
| Database schema | Done |
| Rust CLI: fetch teams | Done |
| Rust CLI: fetch game types | Done |
| PHP command: fetch teams | Done |
| Symfony web app skeleton | Done |
| HTTP API / web routes | Not started |
| User authentication | Schema only |
| Matchup tracking | Schema only |
| Tests | Not started |
