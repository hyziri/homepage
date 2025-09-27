# Autumn Homepage
The homepage for EVE Online corporation [The Order of Autumn](https://zkillboard.com/corporation/98785281/), part of [Black Rose](https://black-rose.space/) alliance & Phoenix Coalition.

This is a fullstack [Rust](https://www.rust-lang.org/) application built using [Dioxus](https://dioxuslabs.com/).

For contributions & development instructions please see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Production

### Prerequisites

- [Docker](https://docs.docker.com/engine/install/)

### Running for production

1. Copy `.env.example` to `.env` and set the `CONTACT_EMAIL` & `DOMAIN`
2. If your server isn't using a proxy already to handle traffic to your domain, start the provided traefik proxy:

```bash
docker network create traefik
```

```bash
docker compose up -f docker-compose.traefik.yml up -d
```

3. Start the the docker container for the application itself:

```bash
docker compose up up -d
```

If your server is lacking in hardware, consider building the docker image locally on your machine then pushing it to the server. In most cases this is unnecessary and it is perfectly fine to build the image on the server, it'll just take awhile to build.

## Development

### Prerequisites

- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Dioxus-Cli](https://dioxuslabs.com/learn/0.6/getting_started/)

### Running for development

1. Copy `.env.example` to `.env` and set the `CONTACT_EMAIL`
2. Install tailwindcss dependencies:

```bash
bun i
```

3. Run tailwindcss cli to watch for CSS changes:

```bash
bunx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
```

4. In a separate terminal, run the application:

```bash
dx serve
```

5. The application can be found at `http://localhost:8080`

### Modifying Server/Web

The backend server (`#[cfg(feature = "web")]`) is compiled in native Rust while the frontend (`#[cfg(feature = "web")]`) contains Rust is compiled to WASM. These are incompatible and cannot be compiled together hence the usage of feature flags to distinguish between what is backend only and what is frontend only code.

You will need to edit your `rust-analyzer` configuration in order to get code warnings on the proper feature flag depending on what you are currently working on:

**Zed Editor Example:**

Modify your `settings.json` in the `~/.config/zed/` directory:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "features": ["server"]
        }
      }
    }
  }
}
```

### Testing Docker

- [Install Docker](https://docs.docker.com/engine/install/)

To test the docker image build use:

```bash
docker build .
```

To test the compose configuration for the application:

```bash
docker compose -f docker-compose.dev.yml up
```

Then navigate to `localhost:8080`
