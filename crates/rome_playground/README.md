# Playground

A simple playground for Rome. Right now we use Vite, which is a 
tad ironic, but in the future we can use this as a dogfood for Rome.

## Installation

[wasm-pack](https://github.com/rustwasm/wasm-pack) is 
required to build the playground. It's assumed that you've
cloned the playground repo in the same director as the tools repo.

To build run:
```
wasm-pack build --target web
```

Then, if you want to run in development: `npm run dev`
