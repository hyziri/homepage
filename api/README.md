# Autumn Homepage API

This is the API provider for The Order of Autumn's homepage which supplies Autumn corporation statistics information.

## Development

Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/engine/install/)

1. Run `rustup default stable` to set the rust version
2. Copy `.env.example` from the parent directory to `.env` and set `CONTACT_EMAIL` & `POSTGRES_PASSWORD`
3. Copy the value you set for `POSTGRES_PASSWORD` & replace the `POSTGRES_PASSWORD` within the `DATABASE_URL` used for local testing
4. Run `docker compose -f docker-compose.dev.yml up -d` to start a development postgres database instance
5. Run the application with `cargo run`

API documentation can be found at `http://localhost:8000`

For docker you can run the application with `docker compose up -d --build`
- `-d` to run detached from terminal
- `--build` to rebuild image if there are any changes

### Database Migrations

After modifying migrations, run these commands in this order:

Apply migrations to database
```
sea-orm-cli migrate
```

Generate entities based upon database tables
```
sea-orm-cli generate entity -o ./entity/src/entities/ --date-time-crate chrono
````
