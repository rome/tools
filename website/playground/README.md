# Playground

A simple playground for Rome. Right now we use Vite, which is a tad ironic, but
in the future we can use this as a dogfood for Rome.

## Installation

[`wasm-pack`](https://github.com/rustwasm/wasm-pack) is required to build the
playground. Follow the
[instructions](https://rustwasm.github.io/wasm-pack/installer/) to install
`wasm-pack`.

`cd` to `website/playground` directory and build the Rust bindings:

```shell
cd website/playground
pnpm build:wasm
```

Then, if you want to run in development

```shell
pnpm install # if you haven't ran this yet
pnpm start
```

## Tips

1. You could add this option in `./cargo.toml`

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

this could accelerate your wasm compiler time during development, but don't
forget to remove it when you make a pull request.
