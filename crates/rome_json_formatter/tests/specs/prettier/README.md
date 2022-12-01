# Prettier Test Suite

These test snapshots were extracted from the
[prettier/prettier](https://github.com/prettier/prettier) repository

# Usage

These tests are run as part of the `rome_js_formatter` test suite but can be
explicitely called with `cargo test -p rome_js_formatter --test prettier_tests`

Setting the `REPORT_PRETTIER=1` environment variable when running these tests
will emit a `report.md` file containing an exhaustive difference between the
output of `rome_js_formatter` and Prettier's own snapshots

# Updating

Prettier is using Jest to run snapshot tests, and a node.js script is needed to
extract these snapshots into plain files for use in the Rust tests. To update
the snapshots:

1. Clone the Prettier git repository locally
2. Remove all the directories inside
   `crates/rome_js_formatter/tests/specs/prettier` to ensure all obsolete tests are
   removed
3. Go to `crates/rome_js_formatter/tests/specs/prettier` directory
4. Install prettier ``pnpm install``
5. Run
   `node crates/rome_js_formatter/tests/specs/prettier/prepare_tests.js <prettier root directory>`
