# IEEE-Registry

[![Continuous integration](https://github.com/AdamIsrael/ieee-registry/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/AdamIsrael/ieee-registry/actions/workflows/ci.yaml)

The IEEE-Registry crate provides a locally cached copy of the IEEE Registration Authority's public listings in the current users `~/.local/share/ieee/` directory in order to be used for lookup purposes.

The crate provides two ways of caching the IEEE registry public listings:

```bash
$ ieee-registry
Caching IEEE registry file(s)...
✔ /home/adam/.local/share/ieee/cid.csv
✔ /home/adam/.local/share/ieee/eth.csv
✔ /home/adam/.local/share/ieee/iab.csv
✔ /home/adam/.local/share/ieee/mam.csv
✔ /home/adam/.local/share/ieee/man.csv
✔ /home/adam/.local/share/ieee/opid.csv
✔ /home/adam/.local/share/ieee/oui.csv
✔ /home/adam/.local/share/ieee/oui36.csv
```

or programatically:

```rust
use ieee_registry::*;

// Get the path to oui.csv, downloading it if necessary.
let oui_path = get_oui_path();
```

Currently, we check the age of the downloaded file. If it's more than 30 days old, a new copy will be downloaded.
