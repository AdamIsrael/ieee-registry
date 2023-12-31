# IEEE-Registry

[![Continuous integration](https://github.com/AdamIsrael/ieee-registry/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/AdamIsrael/ieee-registry/actions/workflows/ci.yaml)

The IEEE-Registry crate provides a locally cached copy of the IEEE Registration Authority's public listings in the current users `~/.local/share/ieee/` directory in order to be used for lookup purposes.


```rust
use ieee_registry::ieee::*;

```
