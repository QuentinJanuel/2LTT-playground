# 2LTT Playground

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
