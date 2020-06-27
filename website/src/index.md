---
layout: layouts/base.njk
showHero: true
---

**Rome** is a JavaScript toolchain. It unifies functionality that has previously been completely separate tools. **Rome** is a linter, compiler, bundler, and [more](#responsibilities).

Project such as [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [Webpack](https://webpack.js.org/), [Jest](https://jestjs.io/), and related tooling have a large overlap in implementation and responsibilities.

There is value in these being a single tool. Building upon a shared base allows us to provide a cohesive experience for processing JavaScript, displaying errors, parallelizing work, caching, and configuration.

**Rome** is opinionated. We aim to have minimal configuration. By using Rome you accept the conventions we have set. Read more about our philosophy [here](/contributing/philosophy).

**Rome** supports regular JavaScript, TypeScript and JSX.

**Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com). **Rome** is maintained by a [team of contributors](/contributing/team).

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

## Learn more

![One](/static/img/screenshots/1.png)
![Two](/static/img/screenshots/2.png)

<ul class="home-actions">
	<li>
		<a href="/docs/getting-started">Getting Started</a>
	</li>
</ul>

<ul class="home-actions">
	<li>
		<a href="https://github.com/romejs/rome">GitHub</a>
	</li>
	<li>
		<a href="https://opencollective.com/romejs">OpenCollective</a>
	</li>
	<li>
		<a href="https://twitter.com/romejsdev">Twitter</a>
	</li>
</ul>
