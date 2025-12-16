# Caching Implementation

This document describes the caching strategies implemented in the CourtListener Worker.

## Overview

The worker implements multiple layers of caching to improve performance and reduce API calls:

1. **Cloudflare KV** - Distributed edge caching
2. **HTTP Cache Headers** - Browser and CDN caching
3. **Function-level caching** - In-memory caching (optional, via `cached` crate)

## Setup

### 1. Create KV Namespace

```bash
# Create production KV namespace
wrangler kv:namespace create "CACHE"

# Create preview KV namespace  
wrangler kv:namespace create "CACHE" --preview
```

This will output namespace IDs. Update `wrangler.toml` with the actual IDs:

```toml
[[kv_namespaces]]
binding = "CACHE"
id = "your-production-kv-namespace-id"
preview_id = "your-preview-kv-namespace-id"
```

### 2. Deploy

```bash
wrangler deploy
```

## Cache Strategy

### TTL by Endpoint Type

- **Search endpoints** (`/search/`): 5 minutes (300s) - Results can change frequently
- **Dockets** (`/dockets/`, `/docket-alerts`): 15 minutes (900s) - Medium volatility
- **Opinions/Clusters** (`/opinions/`, `/clusters/`): 30 minutes (1800s) - Rarely change
- **Courts/People** (`/courts/`, `/people/`): 1 hour (3600s) - Very stable
- **Default**: 10 minutes (600s)

### Cache Key Generation

Cache keys are generated from:
- Endpoint path (e.g., `/courts/`)
- Query parameters (if present)
- Long query strings (>200 chars) are hashed to avoid KV key length limits

Example keys:
- `courts/` (no query)
- `search/?q=foo&type=o` (short query)
- `search/:q:1234567890` (hashed long query)

### HTTP Cache Headers

Responses include:
- `Cache-Control`: `public, max-age={ttl}, s-maxage={ttl}, stale-while-revalidate={ttl/2}`
- `X-Cache`: `HIT` or `MISS` (indicates if served from KV cache)
- `Vary`: `Accept, Authorization` (for conditional requests)

## Usage

Caching is automatically applied to all API endpoints via `fetch_api_json()`. No code changes needed - it's transparent to endpoint handlers.

## Cache Invalidation

Currently, cache entries expire based on TTL. For manual invalidation:

```bash
# Delete a specific cache key
wrangler kv:key delete "courts/" --namespace-id=your-namespace-id

# List all keys (for debugging)
wrangler kv:key list --namespace-id=your-namespace-id
```

## Monitoring

Cache performance is logged:
- `Cache HIT: {key}` - Response served from cache
- `Cache MISS: {key}` - Cache miss, fetched from API
- `Cache ERROR: {key} - {error}` - Cache operation failed
- `Cached response: {key} (TTL: {ttl}s)` - Successfully cached

## Future Enhancements

- [ ] ETag support for conditional requests
- [ ] Cache warming for popular endpoints
- [ ] Cache analytics/metrics
- [ ] Function-level caching with `cached` crate for expensive computations
- [ ] D1 database for persistent cache metadata


