# boundary-api
[![Crates.io](https://img.shields.io/crates/v/boundary-api)](https://crates.io/crates/boundary-api)
[![docs.rs](https://docs.rs/boundary-api/badge.svg)](https://docs.rs/boundary-api/)
[![license](https://img.shields.io/github/license/tjtelan/boundary-api)](LICENSE)
![Github actions build status](https://github.com/tjtelan/boundary-api-rs/workflows/boundary-api/badge.svg)

Rust client for Hashicorp Boundary

**Incomplete implementation - Not for ready for production!**

## Examples

These assume running Boundary server running in dev mode
```bash
# Start Boundary in dev mode in another terminal window
$ boundary dev
```

## Auth to Boundary server in dev mode
```bash
$ cargo run --example auth-dev-mode
Response { url: "http://127.0.0.1:9200/v1/auth-methods/ampw_1234567890:authenticate", status: 400, headers: {"cache-control": "no-store", "content-type": "application/json", "date": "Thu, 15 Oct 2020 08:24:37 GMT", "content-length": "173"} }
```