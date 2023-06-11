#!/bin/bash

# https://stackoverflow.com/a/246128/3549270
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"

cd corpus/rome_parse_all
cp -r "../../../crates/rome_js_parser/test_data" .
find . -name \*.rast -delete
cd -
cargo fuzz cmin --features rome_parse_all -s none rome_parse_all

echo "Done! You are ready to fuzz."
