<section>

<!-- GENERATED:START(hash:772ef31658a73dda4b29485f5331384d7980a37e,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/website-intro` to update. -->
**Rome** is a linter, compiler, bundler, and [more](#development-status) for JavaScript, TypeScript, JSON, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](/credits) for more information.

**Rome** is maintained by a [team of volunteers](/credits#team) under an established [governance model](https://github.com/romefrontend/rome/blob/main/GOVERNANCE.md).

**Rome** is [MIT licensed](https://github.com/romefrontend/rome/tree/main/LICENSE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/romefrontend/rome/tree/main/CODE_OF_CONDUCT.md).
<!-- GENERATED:END(id:main) -->

{% include action-links.liquid %}

</section>

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
| HTML [#983](https://github.com/romefrontend/rome/issues/983) | <span aria-label="Work in Progress">⌛</span> | <span aria-label="Work in Progress">⌛</span> | <span aria-label="Work in Progress">⌛</span> |
| CSS [#984](https://github.com/romefrontend/rome/issues/984) | ✅ | <span aria-label="Work in Progress">⌛</span> | <span aria-label="Work in Progress">⌛</span> |
| Markdown [#985](https://github.com/romefrontend/rome/issues/985) | <span aria-label="Work in Progress">⌛</span> | <span aria-label="Work in Progress">⌛</span> | <span aria-label="Work in Progress">⌛</span> |
