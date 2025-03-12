# Changelog

<!-- changelogging: start -->

## [0.3.0](https://github.com/nekitdev/refinement-types/tree/v0.3.0) (2025-03-12)

### Changes

- The help message (`H`) was replaced with context (`C`) in refinements.

## [0.2.0](https://github.com/nekitdev/refinement-types/tree/v0.2.0) (2025-03-10)

### Changes

- The entire crate has been rewritten; please refer to the
  [documentation](https://docs.rs/refinement-types).

## [0.1.0](https://github.com/nekitdev/refinement-types/tree/v0.1.0) (2025-03-01)

### Features

- Added `AsRef<T>` for `Refinement<T, P, C>`.

- Added `check` for `Refinement<T, P, C>` that calls `P::check` and returns `Result<(), P::Error>`.
