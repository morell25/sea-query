## Release Notes: sea-query-postgres 0.6.1

*(since sea-query-postgres 0.6.0)*

### New Features

**Jiff type binder support** (via `with-jiff`)

The PostgreSQL binder now supports [Jiff](https://docs.rs/jiff) date/time values behind a new `with-jiff` feature. When enabled, the following `Value` variants are bound to their corresponding PostgreSQL types (including array forms):

- `Value::JiffDate` → `DATE`
- `Value::JiffTime` → `TIME`
- `Value::JiffDateTime` → `TIMESTAMP`
- `Value::JiffTimestamp` → `TIMESTAMPTZ`

The `with-jiff` feature enables `postgres-types/with-jiff-0_2` and `sea-query/with-jiff`.

### Compatibility Notes

This is a patch release of `sea-query-postgres`. The new behavior is gated behind the opt-in `with-jiff` feature; existing feature sets are unaffected.
