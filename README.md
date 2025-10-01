# Orca Whirlpools SDK - Mutex Send Issue Reproduction

## Problem

The SDK uses `std::sync::Mutex` for global configuration:
```rust
pub static WHIRLPOOLS_CONFIG_ADDRESS: Mutex<Pubkey> = Mutex::new(...);
```

When methods like `fetch_whirlpools_by_token_pair` are called inside `tokio::spawn`, it fails because `MutexGuard` doesn't implement `Send`, preventing the async task from being sent between threads.

## Error

```
error[E0277]: `std::sync::MutexGuard<'_, Pubkey>` cannot be sent between threads safely
```

## Reproduction

### Testing the Broken Version (Official SDK)

1. Edit `src/main.rs`:
   ```rust
   // Comment out line 3
   // use rwlock_whirlpools::{...};
   
   // Uncomment line 4
   use orca_whirlpools::{...};
   ```

2. Run:
   ```bash
   cargo check
   ```

3. You'll see the `Send` trait error.

### Testing the Working Version (RwLock Fix)

1. Edit `src/main.rs`:
   ```rust
   // Use line 3 (default)
   use rwlock_whirlpools::{...};
   
   // Comment line 4
   // use orca_whirlpools::{...};
   ```

2. Run:
   ```bash
   cargo run
   ```

3. It compiles and runs successfully.

## The Fix

In `rust-sdk/whirlpool/src/config.rs`, change:

```rust
// Before (Mutex)
pub static WHIRLPOOLS_CONFIG_ADDRESS: Mutex<Pubkey> = Mutex::new(...);

// After (RwLock)
pub static WHIRLPOOLS_CONFIG_ADDRESS: RwLock<Pubkey> = RwLock::new(...);
```

And update all `.try_lock()` calls to `.try_read()` or `.try_write()` accordingly.

## Why RwLock?

- `RwLock` allows multiple concurrent readers (config reads are far more common than writes)
- Better performance for read-heavy workloads
- Still prevents the `Send` issue when values are properly copied from the guard

## Related Issue

See the discussion in the Orca Discord for more context.