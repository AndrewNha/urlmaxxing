# Urlmaxxing

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6&logo=typescript&logoColor=white)
![React](https://img.shields.io/badge/React-20232A?logo=react&logoColor=61DAFB)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=white)

Urlmaxxing is a full-stack application for saving, organizing, and quickly finding useful URLs. Each account has a private bookmark collection protected by a cookie-based session.

## Live application

[Open Urlmaxxing](https://urlmaxxing.vercel.app/)

![Urlmaxxing homepage](docs/images/home.png)

## Features

- Account registration and login with HttpOnly cookie sessions
- Change username and password or permanently delete an account
- Private bookmark collections scoped to the authenticated user
- Create, view, edit, and delete bookmarks
- Optional tags and search by title, URL, or tag
- Responsive interface with light and dark themes
- Loading, empty, success, validation, and error states
- Configurable per-IP rate limits for the API, login, and registration
- Password hashing with bcrypt and JWT session validation
- Automatic SQLx database migrations during API startup
- Docker Compose setup for the API and PostgreSQL
- Unit, integration, component, and end-to-end test suites

## Technology stack

| Area | Technologies |
| --- | --- |
| Front end | React, TypeScript, Vite, Tailwind CSS, React Router, Framer Motion |
| Back end | Rust, Axum, Tokio, SQLx, bcrypt, JSON Web Tokens |
| Database | PostgreSQL 16 |
| Infrastructure | Docker, Docker Compose, GitHub Actions |
| Testing | Rust tests, SQLx integration tests, Vitest, Testing Library, Playwright |

## Run locally

The recommended development setup runs PostgreSQL and the Rust API in Docker, while Vite runs the front end locally for fast reloads.

### Prerequisites

- [Git](https://git-scm.com/)
- [Docker](https://docs.docker.com/get-docker/) with Docker Compose v2
- [Node.js 22](https://nodejs.org/) and npm

### 1. Clone the repository

```bash
git clone https://github.com/AndrewNha/urlmaxxing.git
cd urlmaxxing
```

### 2. Configure and start the API

Create the local environment file:

```bash
cd back-end
cp .env.example .env
```

Open `back-end/.env` and replace `JWT_SECRET` with a long, random value. The other defaults are ready for local development.

Build and start PostgreSQL and the API:

```bash
docker compose up --build -d
```

The API waits for PostgreSQL to become healthy and then applies all pending SQLx migrations automatically. Confirm that it is running:

```bash
curl http://localhost:3000/health
```

Expected response:

```json
{"status":"OK"}
```

### 3. Start the front end

In another terminal, from the repository root:

```bash
cd front-end
npm ci
npm run dev
```

Open [http://localhost:5173](http://localhost:5173) in your browser.

## Local services

| Service | Address |
| --- | --- |
| Front end | `http://localhost:5173` |
| API | `http://localhost:3000` |
| Health check | `http://localhost:3000/health` |
| PostgreSQL | `localhost:5432` |

## Environment variables

The back end reads its configuration from `back-end/.env` when Docker Compose starts.

| Variable | Default | Purpose |
| --- | --- | --- |
| `POSTGRES_USER` | `user` | PostgreSQL user created by the container |
| `POSTGRES_PASSWORD` | `password` | PostgreSQL password for local development |
| `POSTGRES_DB` | `database` | PostgreSQL database name |
| `DATABASE_URL` | `postgres://user:password@db:5432/database` | API connection string using the Compose service name |
| `JWT_SECRET` | example value | Secret used to sign session tokens; replace it locally |
| `COOKIE_SECURE` | `false` | Set to `true` when the application is served over HTTPS |
| `FRONTEND_ORIGIN` | `http://localhost:5173` | Browser origin allowed by CORS |
| `RATE_LIMIT_GENERAL_PER_MINUTE` | `120` | Maximum requests per IP per minute across API routes except the health check |
| `RATE_LIMIT_LOGIN_PER_MINUTE` | `5` | Maximum login attempts per IP per minute |
| `RATE_LIMIT_REGISTER_PER_HOUR` | `5` | Maximum registrations per IP per hour |

The API returns HTTP `429 Too Many Requests` when a client exceeds a configured quota.

## Docker commands

Run these commands inside `back-end`.

View container status:

```bash
docker compose ps
```

Follow API and database logs:

```bash
docker compose logs -f
```

Rebuild after a back-end dependency or source change:

```bash
docker compose up --build -d
```

Stop the containers while preserving database data:

```bash
docker compose down
```

Delete the containers and the PostgreSQL volume:

```bash
docker compose down -v
```

> [!WARNING]
> The `-v` option permanently deletes the local database and all saved accounts and bookmarks.

## Tests and quality checks

### Back end

Back-end integration tests require a PostgreSQL instance and use SQLx to create isolated test databases. With the Compose database running, execute:

```bash
cd back-end
cargo fmt --check
cargo check --tests
DATABASE_URL=postgres://user:password@localhost:5432/database cargo test
```

### Front end

```bash
cd front-end
npm run typecheck
npm test
npm run build
```

Install the supported Playwright browsers once, then run the end-to-end suite:

```bash
npx playwright install chromium firefox
npm run test:e2e
```

## Project structure

```text
urlmaxxing/
├── back-end/          # Axum API, SQLx migrations, tests, and Docker configuration
├── front-end/         # React application, component tests, and Playwright scenarios
├── docs/images/       # README assets
└── README.md
```

## Security notes

- Session tokens are stored in HttpOnly cookies instead of browser local storage.
- Cookies use `SameSite=Lax`; production deployments must use HTTPS and set `COOKIE_SECURE=true`.
- Passwords are stored as bcrypt hashes.
- Changing a password invalidates the previous session.
- CORS credentials are restricted to the configured front-end origin.
- Application rate limiting helps control abusive clients, but production deployments should also use infrastructure-level protection through a reverse proxy, load balancer, or CDN.
