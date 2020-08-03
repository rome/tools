<section>

<!-- GENERATED:START(hash:8cb63cfa44c2c976570991319738898b0339908a,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/website-intro` to update. -->
**Rome** is a linter, compiler, bundler, and [more](#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [Webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](/credits) for more information.

**Rome** is maintained by a [team of volunteers](/credits#team) under an established [governance model](https://github.com/romefrontend/rome/blob/main/GOVERNANCE.md). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io/) and [Yarn](https://yarnpkg.com/).

**Rome** is [MIT licensed](LICENSE) and moderated under the [Contributor Covenant Code of Conduct](github.com/romefrontend/rome/tree/main/CODE_OF_CONDUCT.md).
<!-- GENERATED:END(id:main) -->

{% include docs/cli-screenshots/intro.md %}

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
		<a href="https://discord.gg/rome">
			{% include layouts/svg/discord.svg %} Discord
		</a>
	</li>
	<li>
		<a href="https://twitter.com/romefrontend">
			{% include layouts/svg/twitter.svg %} Twitter
		</a>
	</li>
</ul>

</section>

## History

> The history of Rome starts when I created the project 6to5, that would later become Babel. A JavaScript transpiler that converted ES6 to ES5.
>
> In early 2015, we renamed 6to5 to [Babel](https://babeljs.io). The name change was motivated by the desire to turn the project into something more general that could be used to develop other tooling on top of. I left the project and those plans never materialized.
>
> Rome is the spiritual successor of that idea, except rather than exposing a large public API for other tools to use, we are building them all in one place, batteries included.
>
> I built Rome in private over the course of the last few years while working at Facebook, largely during my personal time. The project was open sourced at the beginning of 2020 and continues as a community project.

— [Sebastian McKenzie](https://twitter.com/sebmck), author of **Babel** and **Rome**. [Read more](https://twitter.com/sebmck/status/1063574500938117120).

**Rome** has a logo of a Roman arch, one of the most influential patterns in architecture. It symbolizes a strong foundation, allowing you to build large projects without worrying and struggling with tooling.

## Development Status

**Rome is currently only supported as a [linter](#linting) for JavaScript and TypeScript.** We are actively working on support for other languages.

Once our usage as a linter has matured we will work on releasing the other parts of Rome and expand beyond linting. Significant implementation has already been completed for other functionality such as bundling and testing.

### Language Support

| Language | Parsing | Formatting | Linting |
| - | - | - | - |
| JavaScript | ✅ | ✅ | ✅ |
| &mdash; TypeScript | ✅ | ✅ | ✅ |
| &mdash; JSX  | ✅ | ✅ | ✅ |
| JSON | ✅ | ✅ | |
| &mdash; [RJSON](#rome-json) | ✅ | ✅ | |
| HTML | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) |
| CSS | ✅ | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) |
| Markdown | ✅ | ✅ | ⌛ [#123](https://github.com/romefrontend/rome/issues/123) |
