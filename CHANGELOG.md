# Changelog

<!-- changelogging: start -->

## [0.1.0](https://github.com/nekitdev/refinement-types/tree/v0.1.0) (2025-03-01)

### Features

- Added `AsRef<T>` for `Refinement<T, P, C>`.

- Added `check` for `Refinement<T, P, C>` that calls `P::check` and returns `Result<(), P::Error>`.
