<section>

<!-- GENERATED:START(hash:07631a12e2ec218508178887bce2cd5e94c5654b,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/website-intro` to update. -->
**Rome** is a linter, compiler, bundler, and [more](#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [Webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](/credits) for more information.

**Rome** is maintained by a [team of volunteers](/credits#team) under an established [governance model](https://github.com/romefrontend/rome/blob/main/GOVERNANCE.md).

**Rome** is [MIT licensed](LICENSE) and moderated under the [Contributor Covenant Code of Conduct](github.com/romefrontend/rome/tree/main/CODE_OF_CONDUCT.md).
<!-- GENERATED:END(id:main) -->

{% include action-links.liquid %}

</section>

## History

**Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io/) and [Yarn](https://yarnpkg.com/):

> #### Inception
>
> The original project goal of Babel was to expand beyond compilation and be a more general base for other JavaScript tools. All language tooling generally has the same constraints and similar responsibilities, so it seemed silly that every tool was rolling their own configuration format, caching, parallelization model, parsing and more. These plans never materialized after I left the project, but the idea still stuck.
>
> Rome is the spiritual successor of that idea, except rather than exposing a large public API for other tools to be built on, we are building them all in one place, with batteries included. We support frontend languages other than JavaScript too, those that can benefit the most from tight integration.
>
> #### Development
>
> Development began in January 2018 while I was employed at Facebook. It was developed as a side project until 2019 when I was able to work on it full-time. I have since left Facebook, and Rome was open sourced in February 2020, where it continues to be driven entirely as a community project.

## Development Status

**Rome is currently only supported as a [linter](#linting) for JavaScript and TypeScript.** We are actively working on support for other languages.

Once our usage as a linter has matured we will work on releasing the other parts of Rome and expand beyond linting. **Significant implementation already exist for most functionality.**

We plan on covering the following areas:

 - Bundling
 - Compiling
 - Documentation Generation
 - Formatting
 - Linting
 - Minification
 - Testing
 - Type Checking
 - ... and more

### Language Support

| Language | Parsing | Formatting | Linting |
| - | - | - | - |
| JavaScript | ✅ | ✅ | ✅ |
| &mdash; TypeScript | ✅ | ✅ | ✅ |
| &mdash; JSX  | ✅ | ✅ | ✅ |
| JSON | ✅ | ✅ | |
| &mdash; [RJSON](#rome-json) | ✅ | ✅ | |
| HTML | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) |
| CSS | ✅ | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) |
| Markdown | ✅ | ✅ | <span aria-label="Work in Progress">⌛</span> [#123](https://github.com/romefrontend/rome/issues/123) |
