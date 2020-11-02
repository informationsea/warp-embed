# warp-embed
[![Build](https://github.com/informationsea/warp-embed/workflows/Build/badge.svg)](https://github.com/informationsea/warp-embed/actions)
[![GitHub](https://img.shields.io/github/license/informationsea/websockify-rs)](https://github.com/informationsea/websockify-rs)
[![GitHub top language](https://img.shields.io/github/languages/top/informationsea/websockify-rs)](https://github.com/informationsea/websockify-rs)
[![Crates.io](https://img.shields.io/crates/v/warp-embed)](https://crates.io/crates/warp-embed)
[![Docs.rs](https://docs.rs/warp-embed/badge.svg)](https://docs.rs/warp-embed)

Serve [embedded file](https://crates.io/crates/rust-embed) with [warp](https://crates.io/crates/warp)

```rust
use warp::Filter;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data"]
struct Data;

let data_serve = warp_embed::embed(&Data);
```