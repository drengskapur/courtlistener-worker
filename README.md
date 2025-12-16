# CourtListener Worker

A Cloudflare Worker written in Rust for interacting with the CourtListener API, with complete type definitions for all API responses.

## Features

- 🦀 100% Rust implementation
- 📦 Complete type definitions for CourtListener API v4.4.0
- ⚡ Cloudflare Workers runtime
- 🔒 Type-safe API interactions
- 📚 Well-documented codebase
- 📖 Interactive API documentation (Swagger UI, ReDoc, Scalar)
- 🌊 Streaming support for large audio files
- 🔔 Webhook receiver endpoint

## Getting Started

### Prerequisites

- Rust (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Node.js and npm (for Wrangler)
- Wrangler CLI: `npm install -g wrangler`
- CourtListener API token (optional, for authenticated endpoints)

### Setup

1. **Copy `.env.example` to `.env`** and add your API token:

   ```bash
   cp .env.example .env
   # Then edit .env and add your actual token
   ```

   Or create `.env` manually:

   ```bash
   echo "COURTLISTENER_API_TOKEN=your_token_here" > .env
   ```

2. **Optional: Set API base URL** (for testing different API versions):

   ```bash
   # In .env file:
   COURTLISTENER_API_BASE_URL=https://www.courtlistener.com/api/rest/v5
   ```

   Defaults to `v4` path (API version 4.4.0) if not set.
   
   **Note:** The API path uses `/api/rest/v4/` but the actual API version is 4.4.0.
   The path remains "v4" for all 4.x versions.

3. **Set Wrangler secret** (for production):

   ```bash
   npx wrangler secret put COURTLISTENER_API_TOKEN
   ```

### Development

```bash
# Run locally (reads .env automatically)
npx wrangler dev

# Build
npx wrangler deploy --dry-run

# Deploy
npx wrangler deploy
```

### Endpoint Coverage Check

Check which endpoints are supported vs available in the live API:

```bash
curl http://localhost:8787/check-endpoints
```

Returns JSON with comparison of our endpoints vs the live CourtListener API.

### API Documentation

The worker includes interactive API documentation powered by the complete CourtListener API OpenAPI specification:

- **Static (default)**: `/docs/openapi.json` - Serves the embedded OpenAPI spec (fast, cached)
- **Dynamic (on-demand)**: `/docs/openapi.json?fresh=true` - Generates a fresh spec by querying the live CourtListener API (slower, always up-to-date)

- **Swagger UI**: `/docs` or `/docs/swagger` - Full-featured API explorer
  - Add `?fresh=true` to use dynamically generated spec: `/docs/swagger?fresh=true`
- **ReDoc**: `/docs/redoc` - Beautiful, responsive documentation
  - Add `?fresh=true` to use dynamically generated spec: `/docs/redoc?fresh=true`
- **Scalar**: `/docs/scalar` - Modern, fast API reference
  - Add `?fresh=true` to use dynamically generated spec: `/docs/scalar?fresh=true`
- **OpenAPI Spec**: `/docs/openapi.json` - Complete OpenAPI 3.0 specification
  - Default: Serves the embedded static spec (fast, cached)
  - With `?fresh=true`: Generates a fresh spec on-demand by querying the live CourtListener API (slower, always up-to-date)

The OpenAPI spec is generated from the live CourtListener API endpoints and includes all available filters, parameters, and response schemas.

**Note**: The dynamic generation (`?fresh=true`) is limited to the first 20 endpoints to avoid worker timeouts. The static spec is embedded at build time from `openapi/v4.4.0/openapi.json`.

### Testing API Endpoints

Once the worker is running, test the endpoints:

```bash
# Health check
curl http://localhost:8787/health

# API root - lists all available APIs
curl http://localhost:8787/api

# Fetch courts (uses API token if set)
curl http://localhost:8787/api/courts

# Fetch courts with filtering and ordering
curl "http://localhost:8787/api/courts?court__jurisdiction=F&order_by=-date_modified"

# Fetch specific court
curl http://localhost:8787/api/courts/scotus

# Fetch opinions with field selection
curl "http://localhost:8787/api/opinions?fields=id,cluster_id,date_filed&page_size=10"

# Fetch opinion clusters
curl http://localhost:8787/api/clusters

# Fetch people with filtering
curl "http://localhost:8787/api/people?fields=id,name_first,name_last&order_by=-date_modified"

# Fetch dockets with complex filtering
curl "http://localhost:8787/api/dockets?court=scotus&id__range=500,1000"

# Alerts - Docket Alerts
curl "http://localhost:8787/api/docket-alerts" \
  -H "Authorization: Token YOUR_TOKEN"
curl -X POST "http://localhost:8787/api/docket-alerts" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "docket=12345"
curl -X PATCH "http://localhost:8787/api/docket-alerts/123" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "alert_type=1"  # Re-up alert

# Alerts - Search Alerts
curl "http://localhost:8787/api/alerts" \
  -H "Authorization: Token YOUR_TOKEN"
curl -X POST "http://localhost:8787/api/alerts" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "name=My Alert&query=q=constitution&rate=dly"

# Search with query parameters (keyword search)
curl "http://localhost:8787/api/search?q=constitution&page_size=5"

# Semantic search (GET)
curl "http://localhost:8787/api/search?q=constitution&semantic=true&type=o"

# Semantic search with embeddings (POST - for privacy)
curl -X POST "http://localhost:8787/api/search?type=o" \
  -H "Content-Type: application/json" \
  -d '{"embedding": [0.123, 0.456, ...]}'

# Search different result types
curl "http://localhost:8787/api/search?q=foo&type=r"  # Federal cases
curl "http://localhost:8787/api/search?q=foo&type=p"  # Judges
curl "http://localhost:8787/api/search?q=foo&type=oa"  # Oral arguments

# Get API metadata (OPTIONS request)
curl -X OPTIONS http://localhost:8787/api/courts

# Use proxy endpoint for any API path
curl http://localhost:8787/api/proxy/schools/

# Webhook-related APIs (via proxy)
curl -X POST "http://localhost:8787/api/proxy/docket-alerts/" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "docket=12345"
curl -X POST "http://localhost:8787/api/proxy/alerts/" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "name=My Alert&query=q=constitution&rate=dly"

# Webhook-related APIs (via proxy)
curl -X POST "http://localhost:8787/api/proxy/docket-alerts/" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "docket=12345"
curl -X POST "http://localhost:8787/api/proxy/alerts/" \
  -H "Authorization: Token YOUR_TOKEN" \
  -d "name=My Alert&query=q=constitution&rate=dly"
```

### Supported Query Parameters

All endpoints support the full CourtListener API query parameters:

- **Filtering**: `?court=scotus&id__gt=100` (supports `__gt`, `__gte`, `__lt`, `__lte`, `__range`, `__startswith`, etc.)
- **Ordering**: `?order_by=-date_modified,id` (use `-` prefix for descending)
- **Field Selection**: `?fields=id,name,date_modified` or `?omit=large_field`
- **Pagination**: `?page=1` or `?cursor=...` (for deep pagination)
- **Counting**: `?count=on` (returns only count, no results)
- **Related Filters**: `?court__jurisdiction=F` (join across APIs)

### Search API Special Features

The Search API (`/api/search`) has additional features:

- **Result Types**: `type=o` (case law, default), `type=r` (federal cases), `type=rd` (PACER filings), `type=d` (dockets), `type=p` (judges), `type=oa` (oral arguments)
- **Semantic Search**: `semantic=true` for GET requests (uses plain language queries)
  - ⚠️ **Only available for case law** (`type=o`)
- **POST with Embeddings**: Send pre-computed embeddings for privacy (768-dimensional vectors)
  - ⚠️ **Only available for case law** (`type=o`)
  - Use CourtListener's Inception microservice with fine-tuned model
- **Highlighting**: `highlight=on` to enable highlighting in snippet fields (uses HTML5 `<mark>` elements)
- **Ordering**: `order_by` parameter (Citegeist sorts by relevancy when ordering by relevancy)
- **Result Counts**: For `type=d` and `type=r`, counts have ±6% error if over 2000 results
- **Default Behavior**: When `type=o`, only published results are returned by default
- **Caching**: Results are cached for 10 minutes (use Alert API for monitoring new results)
- **Note**: Search API doesn't support OPTIONS requests (unlike other APIs)
- **Note**: Search API uses search engine (not database), so filtering/ordering differs from other APIs

### Audio File Streaming

The worker supports streaming large audio files efficiently using Cloudflare Workers' built-in streaming capabilities:

- **Stream by URL**: `GET /api/audio/stream?url=https://...` - Stream directly from a URL
- **Stream by ID**: `GET /api/audio/stream?id=12345` - Fetch metadata first, then stream the enhanced MP3 (local_path_mp3)

**Features**:

- ✅ Automatic streaming for large files (no memory limits)
- ✅ Supports both original files (download_url) and enhanced MP3s (local_path_mp3)
- ✅ Proper Content-Type and Content-Disposition headers
- ✅ CORS support for browser downloads
- ✅ 24-hour cache headers for performance

**Example**:

```bash
# Get audio metadata
curl "http://localhost:8787/api/audio/12345" -H "Authorization: Token YOUR_TOKEN"

# Stream the enhanced MP3 file
curl "http://localhost:8787/api/audio/stream?id=12345" -o audio.mp3
```

### Alerts

The worker provides dedicated endpoints for managing alerts:

**Docket Alerts** (`/api/docket-alerts`):

- `GET /api/docket-alerts` - List your docket alert subscriptions
- `POST /api/docket-alerts` - Create a new docket alert subscription
- `GET /api/docket-alerts/:id` - Get a specific docket alert
- `PATCH /api/docket-alerts/:id` - Update a docket alert (e.g., re-up it)
- `DELETE /api/docket-alerts/:id` - Delete a docket alert

**Search Alerts** (`/api/alerts`):

- `GET /api/alerts` - List your search alert subscriptions
- `POST /api/alerts` - Create a new search alert
- `GET /api/alerts/:id` - Get a specific search alert
- `PATCH /api/alerts/:id` - Update a search alert
- `DELETE /api/alerts/:id` - Delete a search alert

### Webhooks

**Yes, this worker CAN receive webhook events!** Point your domain to this worker and configure webhook URLs in CourtListener.

**Webhook Receiver Endpoint**: `/webhook` or `/webhook/{secret}`

To set up webhook reception:

1. Deploy this worker and point your domain to it (e.g., `https://your-domain.com`)
2. Configure webhook endpoints in your CourtListener account:
   - Endpoint URL: `https://your-domain.com/webhook` (or `/webhook/{secret}` for additional security)
   - Event Type: Choose the events you want to receive
3. The worker will receive POST requests from CourtListener at `/webhook`

**Webhook Security**:

- CourtListener sends webhooks from IPs: `34.210.230.218` or `54.189.59.91`
- Each event includes an `Idempotency-Key` header for deduplication
- Use `/webhook/{secret}` with a long random secret for additional security

**Webhook Event Types**:

- Docket Alert Events - When dockets are updated
- Search Alert Events - When search queries have new results
- Old Docket Alert Events - When alerts are about to be disabled
- RECAP Fetch Events - When RECAP Fetch requests complete
- Pray and Pay Events - When requested documents become available

**Note**: The webhook receiver endpoint currently logs events. You'll need to add your own processing logic (store in database, forward to another service, etc.) in the `receive_webhook` function.

**Other Webhook-related APIs** (accessible via the proxy endpoint):

- `/api/proxy/recap-fetch/` - Monitor RECAP Fetch requests
- `/api/proxy/pray-and-pay/` - Manage Pray and Pay requests

For complete webhook documentation, see the [CourtListener Webhook API docs](https://www.courtlistener.com/api/webhooks/).

See the [CourtListener API documentation](https://www.courtlistener.com/api/rest/v4/) for complete details.

## Project Structure

```
src/
├── lib.rs          # Main worker entry point
├── types.rs        # Core types and enums
├── courts.rs       # Court types
├── opinions.rs     # Opinion and OpinionCluster types
├── people.rs       # Person and Position types
├── dockets.rs      # Docket types
├── citations.rs    # Citation types
└── search.rs       # Search result types
```

## Usage

All types are exported from the crate root:

```rust
use courtlistener_worker::*;

// Use types in your worker
let court = Court {
    id: "us".to_string(),
    name: Some("Supreme Court of the United States".to_string()),
    full_name: Some("Supreme Court of the United States".to_string()),
    abbreviation: Some("SCOTUS".to_string()),
};
```

## Security Considerations

- **API Token**: Store your `COURTLISTENER_API_TOKEN` securely using Wrangler secrets in production
- **CORS**: By default, CORS allows all origins (`*`). For production, set `CORS_ALLOWED_ORIGINS` environment variable to restrict access
- **Proxy Endpoint**: The `/api/proxy/*` endpoint forwards requests to CourtListener API. Path validation prevents SSRF attacks, but use with caution
- **Rate Limiting**: This worker does not implement rate limiting. Consider adding rate limiting at the Cloudflare level or in your application
- **Error Messages**: Error messages are sanitized to avoid leaking sensitive information

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details
