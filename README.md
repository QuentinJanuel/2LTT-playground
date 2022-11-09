# 2LTT Playground

## Introduction
This is a playground for our implementation of the 2LTT formal system. It is based on the [2LTT paper](https://andraskovacs.github.io/pdfs/2ltt.pdf).

## Prerequisites
- [Node.js](https://nodejs.org/en/)
- [npm](https://www.npmjs.com/)
- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

## Setup
First compile the wasm module:
```bash
cd wasm
wasm-pack build
```

Then install the npm dependencies:
```bash
cd ..
npm install
```

## Run
Finally, run the app:
```bash
npm start
```
or build it:
```bash
npm run build
```

## Authors
- Quentin Januel
- Zakaria Djebbes

## License
[MIT](LICENSE)
