**Rome** is a linter, compiler, bundler, and [more](https://romefrontend.dev/#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS. Read more about our [language support](https://romefrontend.dev/docs/language-support).

**Rome** unifies functionality that has previously been completely separate tools. Most frontend tooling have a significant overlap in responsibilities and implementation. There is value in these being a single tool. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has been built from scratch without the usage of existing libraries. **Rome** contains no third-party library dependencies. See [credits](/credits) for a full list of project inspiration and forked code.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](https://romefrontend.dev/about#philosophy).

{% include homepage-screenshot.md %}

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
