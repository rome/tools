**Rome** is a linter, compiler, bundler, and [more](#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [Webpack](https://webpack.js.org/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](/about#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](/credits) for more information.

**Rome** is maintained by a [team of volunteers](/about#eam). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io/) and [Yarn](https://yarnpkg.com/).

{% include docs/intro-screenshot.md %}

<ul class="home-actions">
	<li>
		<a href="https://github.com/romefrontend/rome">
			{% include layouts/svg/github.svg %} GitHub
		</a>
	</li>
	<li>
		<a href="https://opencollective.com/romefrontend">
			{% include layouts/svg/open-collective.svg %} Open Collective
		</a>
	</li>
	<li>
		<a href="https://twitter.com/romefrontend">
			{% include layouts/svg/twitter.svg %} Twitter
		</a>
	</li>
</ul>


## Development Status

**Rome is currently only supported as a [linter](#linting) for JavaScript and TypeScript.** We are actively working on support for other languages.

Once our usage as a linter has matured we will work on releasing the other parts of Rome and expand beyond linting. Significant implementation has already been completed for other functionality such as bundling and testing.

### Language Support

| Language     | Parsing | Formatting | Linting |
| ------------ | ------- | ---------- | ------- |
| JavaScript   | ✅        | ✅           | ✅        |
| &mdash; TypeScript | ✅        | ✅           | ✅        |
| &mdash; JSX        | ✅        | ✅           | ✅        |
| JSON       | ✅        | ✅           | ❓        |
| HTML         | ✅        | ✅          | ❌        |
| CSS          | ✅        | ✅           | ❌        |
| Markdown     | ✅        | ✅           | ❌        |
