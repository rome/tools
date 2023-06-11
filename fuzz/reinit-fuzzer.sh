#!/bin/bash

# https://stackoverflow.com/a/246128/3549270
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"

cd corpus/rome_format_all
cp -r "../../../crates/rome_js_formatter/test_data" .
find . -name \*.rast -delete
cd -
cargo fuzz cmin --features rome_format_all -s none rome_format_all

cd corpus/rome_format_json
cp -r "../../../crates/rome_json_parser/tests/json_test_suite" .
find . -name \*.rast -delete
cd -
cargo fuzz cmin -s none rome_format_json

echo "Done! You are ready to fuzz."
