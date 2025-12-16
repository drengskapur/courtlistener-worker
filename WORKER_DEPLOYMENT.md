# Worker Deployment Notes

## Proc Macro Limitation

The `#[event]` proc macro from the `worker` crate requires special handling when used with optional dependencies. Currently, there's a known issue where the proc macro may not be found when the worker dependency is optional.

## Solutions

### Option 1: Use as Library Only (Recommended for lawforge)

When using this crate as a library (e.g., in lawforge), disable the worker feature:

```toml
[dependencies]
courtlistener-worker = { path = "../courtlistener-worker", default-features = false }
```

This gives you access to all types, config, and utilities without the worker implementation.

### Option 2: Deploy Worker Separately

If you need to deploy the worker, you can:

1. Create a separate binary crate that depends on this library
2. Use the worker module's routing function directly
3. Or make the worker dependency non-optional when deploying

### Option 3: Fix Proc Macro Issue

The proc macro issue can be resolved by ensuring the worker crate is always available when the feature is enabled. This may require adjusting the Cargo.toml configuration or using a different crate structure.

## Current Status

- ✅ Library compiles and works without worker feature
- ✅ All types, config, and utilities are available
- ⚠️ Worker main function has proc macro issues when worker feature is enabled
- ✅ Worker module routing logic is complete and functional

