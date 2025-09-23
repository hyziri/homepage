# Autumn Homepage
The homepage for EVE Online corporation [The Order of Autumn](https://zkillboard.com/corporation/98785281/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus](https://dioxuslabs.com/) for the frontend and [Axum](https://github.com/tokio-rs/axum) for the backend.

## Running in Production

- Install docker for your respective operating system: <https://docs.docker.com/engine/install/>

Running the Application
1. Copy `.env.example` to `.env` and set `CONTACT_EMAIL`, `DOMAIN`, & `POSTGRES_PASSWORD`
2. If your server isn't already running a proxy, use the provided traefik proxy with

```bash
docker network create traefik
docker compose -f docker-compose.traefik.yml up -d
```

3. Run the following command to start the application with docker:

```bash
docker compose up -d
```

Note: If you change the domain for any reason, you'll need to rebuild the application with:

```bash
docker compose up -d --build
```

The reason why is that the API domain is set for the frontend during compile time rather than run time,
a quirk of the Dioxus frontend that one day is hopefully changed.

## Development

For development instructions, please see the `web` and `api` READMEs for their respective instructions.
