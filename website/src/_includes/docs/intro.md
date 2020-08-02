<!-- GENERATED:START(hash:899d8a542b9b8f85ae60eb2628ac28e1cd6f6687,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/website-intro` to update. -->
**Rome** is a linter, compiler, bundler, and [more](#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [Prettier](https://prettier.io/), [Webpack](https://webpack.js.org/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](/about#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](/credits) for more information.

**Rome** is maintained by a [team of volunteers](/about#eam). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io/) and [Yarn](https://yarnpkg.com/).

**Rome** is [MIT licensed](LICENSE), and the project managed under the [Contributor Covenant Code of Conduct](github.com/romefrontend/rome/tree/main/CODE_OF_CONDUCT.md).
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
		<a href="https://twitter.com/romefrontend">
			{% include layouts/svg/twitter.svg %} Twitter
		</a>
	</li>
</ul>

## History

> The history of Rome starts when I created the project 6to5, that would later become Babel. A JavaScript transpiler that converted ES6 to ES5.
>
> In early 2015, we renamed 6to5 to [Babel](https://babeljs.io). The name change was motivated by the desire to turn the project into something more general that could be used to develop other tooling on top of. I left the project and those plans never materialized.
>
> Rome is the spiritual successor of that idea, except rather than exposing a large public API for other tools to use, we are building them all in one place, batteries included.
>
> I built Rome in private over the course of the last few years while working at Facebook, largely during my personal time. The project was open sourced at the beginning of 2020 and continues as a community project.

— [Sebastian McKenzie](https://twitter.com/sebmck), creator of **Babel** and **Rome**. [Read more](https://twitter.com/sebmck/status/1063574500938117120).

**Rome** derives its name from proverbs such as "All Roads Lead to Rome", "Rome wasn't built in a day" and "When in Rome, do as the Romans do". This refers to the expansive scope and the desire for conformity across the project. It started as a joke at the office.

**Rome** has a logo of a Roman arch, one of the most influential patterns in architecture. It symbolizes a strong foundation, allowing you to build large projects without having to ponder the underlying architecture, and reinventing the wheel.

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
