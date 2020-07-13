---
layout: layouts/homepage.njk
---

<!--
# The Rome Frontend Toolchain
-->

**Rome** is a linter, compiler, bundler, and [more](https://romefrontend.dev/#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS. Read more about our language support [here](https://romefrontend.dev/docs/language-support).

**Rome** unifies functionality that has previously been completely separate tools. Most frontend tooling have a significant overlap in responsibilities and implementation. There is value in these being a single tool. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has been built from scratch without the usage of existing libraries. **Rome** contains no third-party library dependencies.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our project philosophy [here](https://romefrontend.dev/contributing/philosophy).

**Rome** is maintained by a [team of contributors](https://romefrontend.dev/contributing/team). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com).

## Preview

{% rootmd "website/src/_includes/homepage-screenshot.md" %}

## Development status

**Rome is currently only supported as a [linter](/docs/lint).** As Rome's use as a linter stabilizes we will begin polishing the other components for release and usage.

Rome aims to have the following responsibilities:

- Bundling
- Compiling
- Documentation Generation
- Formatting
- Linting
- Minification
- Package Management
- Testing
- Type Checking
