# Terra Firma Scraper

This a web scraper that utilises Servo — a high performance browser engine from Mozilla. You pass in a public URL and a CSS selector through its query parameters, and it returns scraped data.

Forked from https://rust-scraper.now.sh/ but compiled to Wasm to run in TerraFirma.

## Usage

Use it [here](https://wasm.marcopolo.io/scraper.wasm)

## Examples

- [Front Page of Hacker news](https://wasm.marcopolo.io/scraper.wasm/?url=news.ycombinator.com&selector=td.title%3Ea.storylink)
- [Current price of Bitcoin](https://wasm.marcopolo.io/scraper.wasm/?url=coinmarketcap.com/currencies/bitcoin&selector=.cmc-details-panel-price__price)
- [How many new Barts are in service](<https://wasm.marcopolo.io/scraper.wasm/?url=www.bart.gov/about/projects/cars&selector=.content%20table%20td:nth-of-type(3)>)

## Setup

First setup Terrafirma with keybase by following the [hello world tutorial](https://marcopolo.io/code/terrafirma/#terrafirma).

```
cargo build --release
cp target/wasm32-unknown-unknown/release/terrafirma_scraper.wasm /keybase/private/<your-keybase-username>,kbwasm/scraper.wasm
```
