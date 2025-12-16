# CourtListener Worker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![Security Policy](https://img.shields.io/badge/Security-Policy-blue.svg)](SECURITY.md)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://drengskapur.github.io/courtlistener-worker/)

> A Cloudflare Worker written in Rust for interacting with the CourtListener API, with complete type definitions for all API responses.

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)
- [License](#license)

## Background

This worker provides a type-safe Rust interface to the CourtListener API, running on Cloudflare Workers. It includes complete type definitions for all API responses and supports all CourtListener API features including search, alerts, webhooks, and audio streaming.

## Install

### Prerequisites

- Rust (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Node.js and npm (for Wrangler)
- Wrangler CLI: `npm install -g wrangler`

### Setup

1. Copy `.env.example` to `.env` and add your API token:

   ```bash
   cp .env.example .env
   ```

2. (Optional) Set API base URL:

   ```bash
   COURTLISTENER_API_BASE_URL=https://www.courtlistener.com/api/rest/v4
   ```

3. For production, set Wrangler secret:

   ```bash
   npx wrangler secret put COURTLISTENER_API_TOKEN
   ```

## Usage

### Development

```bash
# Run locally
npx wrangler dev

# Deploy
npx wrangler deploy
```

### Basic Example

```rust
use courtlistener_worker::*;

let court = Court {
    id: "us".to_string(),
    name: Some("Supreme Court of the United States".to_string()),
    // ...
};
```

### API Endpoints

- `/api/*` - All CourtListener API endpoints
- `/docs` - Interactive API documentation (Swagger UI, ReDoc, Scalar)
- `/health` - Health check
- `/check-endpoints` - Endpoint coverage comparison

See the [API documentation](#api-documentation) for complete details.

## API Documentation

Interactive API documentation is available at `/docs` when the worker is running:

- **Swagger UI**: `/docs` or `/docs/swagger`
- **ReDoc**: `/docs/redoc`
- **Scalar**: `/docs/scalar`
- **OpenAPI Spec**: `/docs/openapi.json`

The OpenAPI spec is auto-generated from the live CourtListener API. Use `?fresh=true` to generate a fresh spec on-demand.

For complete API details, see the [CourtListener API documentation](https://www.courtlistener.com/api/rest/v4/).

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).

## Security

For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.

## License

MIT © CourtListener Worker Contributors

See [LICENSE](LICENSE) for details.
