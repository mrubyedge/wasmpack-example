# `mrubyedge/wasmpack-example`

## About

This is a sample project for generating npm packages with mruby/edge + wasm-pack.
It includes an example of using the generated npm package from Vite under the [`examples/`](examples/) directory.
  
## How to Build

Simply run wasm-pack to generate the npm package under the `pkg` directory.

```sh
$ wasm-pack build --target web
```

To run the Vite example, go to the `examples/mrubyedge-example` directory and install dependencies with npm or yarn, then run the development server.

```sh
$ cd examples/mrubyedge-example
$ npm install
$ npm run dev
```

### About Ruby Script

- is located in `src/mruby/` directory.
- `build.rs` compiles the Ruby script into mrb files during the build process.
- `build.rs` automatically generates the mrb files to include, so no additional tools need to be installed (such as `mrbc`).
- To change the functions you want to export, modify the functions with the `#[wasm_bindgen]` annotation in `src/lib.rs`.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Welcome pull requests!