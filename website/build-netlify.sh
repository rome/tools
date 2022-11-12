#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
pnpm build:wasm
pnpm install
pnpm build:js
