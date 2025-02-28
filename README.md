# `refinement-types`

[![License][License Badge]][License]
[![Version][Version Badge]][Crate]
[![Downloads][Downloads Badge]][Crate]
[![Test][Test Badge]][Actions]

> *Refinement types.*

## Installation

### `cargo`

You can add `refinement-types` as a dependency with the following command:

```console
$ cargo add refinement-types
```

Or by directly specifying it in the configuration like so:

```toml
[dependencies]
refinement-types = "0.0.0"
```

Alternatively, you can add it directly from the source:

```toml
[dependencies.refinement-types]
git = "https://github.com/nekitdev/refinement-types.git"
```

## Examples

### Library

```rust
// lib.rs

#![no_std]

use core::fmt;

use refinement_types::{Refinement, int::U8Closed, length::Closed, logic::And, str::IsAscii};

/// Represents device names.
pub type Name<'n> = Refinement<&'n str, And<Closed<1, 32>, IsAscii>>;

/// Represents device charge, in percentage.
pub type Charge = Refinement<u8, U8Closed<1, 100>>;

/// Represents devices.
#[derive(Debug)]
pub struct Device<'d> {
    /// The name of the device.
    name: Name<'d>,
    /// The charge of the device.
    charge: Charge,
}

impl fmt::Display for Device<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{name}: {charge}%",
            name = self.name,
            charge = self.charge
        )
    }
}

impl<'d> Device<'d> {
    /// Constructs [`Self`].
    pub fn new(name: Name<'d>, charge: Charge) -> Self {
        Self { name, charge }
    }
}
```

### Binary

```rust
// main.rs

use device::{Charge, Device, Name};
use refinement_types::MessageError;

fn main() -> Result<(), MessageError> {
    let charge = Charge::refine(69)?;
    let name = Name::refine("nekit")?;

    let device = Device::new(name, charge);

    println!("{device}"); // nekit: 69%

    Ok(())
}
```

## Documentation

You can find the documentation [here][Documentation].

## Support

If you need support with the library, you can send an [email][Email].

## Changelog

You can find the changelog [here][Changelog].

## Security Policy

You can find the Security Policy of `refinement-types` [here][Security].

## Contributing

If you are interested in contributing to `refinement-types`, make sure to take a look at the
[Contributing Guide][Contributing Guide], as well as the [Code of Conduct][Code of Conduct].

## License

`refinement-types` is licensed under the MIT License terms. See [License][License] for details.

[Email]: mailto:support@nekit.dev

[Discord]: https://nekit.dev/chat

[Actions]: https://github.com/nekitdev/refinement-types/actions

[Changelog]: https://github.com/nekitdev/refinement-types/blob/main/CHANGELOG.md
[Code of Conduct]: https://github.com/nekitdev/refinement-types/blob/main/CODE_OF_CONDUCT.md
[Contributing Guide]: https://github.com/nekitdev/refinement-types/blob/main/CONTRIBUTING.md
[Security]: https://github.com/nekitdev/refinement-types/blob/main/SECURITY.md

[License]: https://github.com/nekitdev/refinement-types/blob/main/LICENSE

[Crate]: https://crates.io/crates/refinement-types
[Documentation]: https://docs.rs/refinement-types

[License Badge]: https://img.shields.io/crates/l/refinement-types
[Version Badge]: https://img.shields.io/crates/v/refinement-types
[Downloads Badge]: https://img.shields.io/crates/dr/refinement-types
[Test Badge]: https://github.com/nekitdev/refinement-types/workflows/test/badge.svg
