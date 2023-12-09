# Advent of code 2023
Chrome browser extension with wasm

## Build Rust code
In `adv2023-lib` folder run `wasm-pack build --target web`, this will generate a `pkg` folder with all the files you need.
Move then `pkg/adv2023_lib.js` and `pkg/adv2023_lib_bg.wasm` in the folder `src/wasm/`, or change parameters of `wasm-pack` command, like:
- `wasm-pack build --target web --out-dir ../src/wasm --no-pack --no-typescript`

## Build extension
In the root run `pnpm build`

## To test it
- Build the extension
- Go to `chrome://extensions/`
- Use _"Load unpacked"_ button
- Choose the `dist` directory of this repository
