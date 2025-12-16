# 🏗️ Proposed Idiomatic Rust Structure for CourtListener Worker

## Current Structure
```
src/
├── lib.rs (1480 lines - everything in one file)
├── cache.rs
├── types.rs
├── courts.rs, opinions.rs, etc. (type definitions)
└── alerts.rs, audio.rs, etc. (type definitions)
```

## Proposed Structure (Idiomatic Rust)

```
src/
├── lib.rs                    # Minimal entry point (~100 lines)
│                             # - Module declarations
│                             # - Router setup
│                             # - Main handler
│
├── config.rs                 # ✅ DONE - Configuration constants
├── errors.rs                 # ✅ DONE - Centralized error types
├── utils.rs                  # ✅ DONE - Internal utilities
│
├── api/                      # ✅ DONE - API client layer
│   ├── mod.rs
│   ├── client.rs             # High-level API client
│   └── request.rs            # Low-level request building
│
├── handlers/                 # Route handlers (organized by concern)
│   ├── mod.rs
│   ├── api.rs                # API endpoint handlers (courts, opinions, etc.)
│   ├── docs.rs               # Documentation handlers (OpenAPI, Swagger, etc.)
│   ├── proxy.rs              # Generic proxy handler
│   └── webhooks.rs           # Webhook receiver
│
├── cache/                    # Caching module
│   ├── mod.rs                # Re-export cache functions
│   └── kv.rs                 # KV cache implementation
│
└── types/                    # Domain types (better organization)
    ├── mod.rs                # Re-export all types
    ├── courts.rs
    ├── opinions.rs
    ├── people.rs
    ├── dockets.rs
    ├── search.rs
    ├── citations.rs
    ├── audio.rs
    └── common.rs             # Shared types (responses, pagination, etc.)
```

## Benefits

1. **Separation of Concerns**: Each module has a single responsibility
2. **Maintainability**: Easy to find and modify specific functionality
3. **Testability**: Modules can be tested independently
4. **Scalability**: Easy to add new endpoints or features
5. **Idiomatic Rust**: Follows Rust community best practices
6. **Clean Public API**: `lib.rs` is minimal and focused

## Migration Strategy

### Phase 1: Foundation (✅ DONE)
- [x] Create `errors.rs`
- [x] Create `config.rs`
- [x] Create `utils.rs`
- [x] Create `api/` module

### Phase 2: Handlers (In Progress)
- [ ] Extract API handlers to `handlers/api.rs`
- [ ] Extract docs handlers to `handlers/docs.rs`
- [ ] Extract proxy handler to `handlers/proxy.rs`
- [ ] Extract webhook handler to `handlers/webhooks.rs`

### Phase 3: Types Organization
- [ ] Create `types/` module structure
- [ ] Move type definitions to appropriate files
- [ ] Update imports throughout codebase

### Phase 4: Cache Module
- [ ] Move `cache.rs` to `cache/mod.rs`
- [ ] Split into logical submodules if needed

### Phase 5: Cleanup
- [ ] Update `lib.rs` to be minimal entry point
- [ ] Remove duplicate code
- [ ] Update all imports
- [ ] Run tests and fix any issues

## Example: New lib.rs Structure

```rust
//! CourtListener API Cloudflare Worker

use worker::*;

// Module declarations
mod api;
mod cache;
mod config;
mod errors;
mod handlers;
mod types;
mod utils;

// Re-exports for convenience
pub use types::*;

// Main entry point
#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    worker::console_log!("Request: {} {}", req.method(), req.path());
    
    handlers::setup_routes()
        .run(req, env)
        .await
}
```

## Next Steps

Would you like me to:
1. **Continue with full refactoring** - Complete all phases systematically
2. **Incremental approach** - Do one phase at a time, test, then continue
3. **Show example first** - Create one complete handler module as example

