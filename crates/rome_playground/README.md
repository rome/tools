# Playground

A simple playground for Rome. Right now we use Vite, which is a 
tad ironic, but in the future we can use this as a dogfood for Rome.

## Installation

[`wasm-pack`](https://github.com/rustwasm/wasm-pack) is 
required to build the playground. It's assumed that you've
cloned the playground repo in the same director as the tools repo.

Follow the [instructions](https://rustwasm.github.io/wasm-pack/installer/) to install `wasm-pack`. 

`cd` to `crates/rome_playground` directory.

Once installed, build the Rust bindings:

```shell
wasm-pack build --target web
```

Then, if you want to run in development 

```shell
npm i # if you haven't ran this yet
npm run dev
```
