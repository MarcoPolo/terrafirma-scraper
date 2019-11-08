# Terra Firma Scraper

This is a Scraper written in Rust that compiles to WASM to run in [the cloud](https://marcopolo.io/wasm).

## Usage

Use it [here](https://wasm.marcopolo.io/scraper.wasm)

## Examples

- [Front Page of Hacker news](https://wasm.marcopolo.io/scraper.wasm/?url=news.ycombinator.com&selector=td.title%3Ea.storylink)
- [Current price of Bitcoin](https://wasm.marcopolo.io/scraper.wasm/?url=coinmarketcap.com/currencies/bitcoin&selector=.cmc-details-panel-price__price)
- [How many new Barts are in service](<https://wasm.marcopolo.io/scraper.wasm/?url=www.bart.gov/about/projects/cars&selector=.content%20table%20td:nth-of-type(3)>)

## Setup

First setup Terrafirma with keybase. Follow the [hello world tutorial](https://marcopolo.io/code/terrafirma/#terrafirma)

```
cargo build --release
cp target/wasm32-unknown-unknown/release/terrafirma_scraper.wasm /keybase/private/<your-keybase-username>,kbwasm/scraper.wasm
```
