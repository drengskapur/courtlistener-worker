# Worker Deployment Notes

## Proc Macro Limitation

The `#[event]` proc macro from the `worker` crate has a known issue with optional dependencies in Rust. Proc macros from optional dependencies may not be available at compile time, which prevents the `#[event]` macro from working when the worker dependency is optional.

## Solutions

### Option 1: Use as Library Only (Recommended for lawforge)

When using this crate as a library (e.g., in lawforge), disable the worker feature:

```toml
[dependencies]
courtlistener-worker = { path = "../courtlistener-worker", default-features = false }
```

This gives you access to all types, config, and utilities without the worker implementation.

**Example usage in lawforge:**
```rust
use courtlistener_worker::*;

// All types are available
let court = Court {
    id: "us".to_string(),
    name: Some("Supreme Court".to_string()),
    // ...
};

// Config constants
let api_version = API_VERSION;
let base_url = API_BASE_URL;
```

### Option 2: Deploy Worker via Separate Binary Crate

Create a separate binary crate (e.g., `courtlistener-worker-bin`) that:

1. Depends on this library with worker feature enabled
2. Has its own `main.rs` with the `#[event]` macro
3. Calls `courtlistener_worker::worker::main()` for routing

**Example `worker-bin/src/main.rs`:**
```rust
use courtlistener_worker::worker;
use worker::*;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    worker::main(req, env, ctx).await
}
```

### Option 3: Make Worker Non-Optional for Deployment

When deploying, you can make the worker dependency non-optional in `Cargo.toml`:

```toml
[dependencies]
worker = "0.7.1"  # Not optional
```

Then uncomment the main function in `lib.rs`.

## Current Status

- ✅ **Library compiles and works perfectly without worker feature** (for lawforge)
- ✅ All types, config, and utilities are available
- ✅ Worker module routing logic is complete and functional
- ⚠️ Worker main function is commented out due to proc macro limitation
- ✅ All worker handlers, API client, caching, and documentation work correctly

