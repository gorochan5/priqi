---
title: "server_address_from_env"
kind: function
path: "iced_beacon::client::server_address_from_env"
crate: "iced_beacon"
module: "iced_beacon::client"
file: "iced_beacon/client/fn.server_address_from_env.md"
docs_url: "https://docs.iced.rs/iced_beacon/client/fn.server_address_from_env.html"
source_file: "src/iced_beacon/client.rs"
source_lines: "221-225"
---

# `iced_beacon::client::server_address_from_env`

```rust
pub fn server_address_from_env() -> String
```

Returns the address of the beacon server in this environment.

The value of the `ICED_BEACON_SERVER_ADDRESS` env variable will be returned, if defined.

Otherwise, a default local server address will be returned.

**Source:** [client.rs](../../src/iced_beacon/client.rs.md) (lines 221-225)
