---
layout: layouts/base.njk
showHero: true
---

**Rome** is a linter, compiler, bundler, and [more](#responsibilities) for JavaScript, HTML, CSS, and Markdown. It unifies functionality that have previously been completely separate tools. We have support for JavaScript flavors such as TypeScript and JSX.

[Babel](https://babeljs.io/), [Webpack](https://webpack.js.org/), [Jest](https://jestjs.io/), [ESLint](https://eslint.org/), and other frontend tooling have a significant overlap in responsibilities and implementation. There is value in these being a single tool.

Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and we aim to have minimal configuration. By using Rome you accept the conventions we have set. Read more about our philosophy [here](https://preview.romejs.dev/contributing/philosophy).

**Rome** is maintained by a [team of contributors](https://preview.romejs.dev/contributing/team). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com).

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

## Learn more

<ul class="home-actions">
	<li>
		<a class="getting-started" href="/docs/getting-started">Get Started</a>
	</li>
</ul>

<ul class="home-actions">
	<li>
		<a href="https://github.com/romejs/rome">
			<svg data-name="github" class="github icon" xmlns="http://www.w3.org/2000/svg" height="32px" width="32px" viewBox="0 0 32 31.21">
				<path class="path-1" d="M16,.29a15.72,15.72,0,0,0-5,30.64c.79.14,1.08-.34,1.08-.76s0-1.36,0-2.67c-4.38.95-5.3-2.11-5.3-2.11A4.16,4.16,0,0,0,5,23.09c-1.42-1,.11-1,.11-1a3.3,3.3,0,0,1,2.41,1.62,3.35,3.35,0,0,0,4.58,1.31,3.31,3.31,0,0,1,1-2.1C9.64,22.56,6,21.21,6,15.19A6.1,6.1,0,0,1,7.59,11a5.68,5.68,0,0,1,.15-4.16s1.32-.42,4.33,1.61a14.87,14.87,0,0,1,7.87,0c3-2,4.32-1.61,4.32-1.61A5.58,5.58,0,0,1,24.41,11,6.06,6.06,0,0,1,26,15.19c0,6-3.68,7.37-7.18,7.76a3.72,3.72,0,0,1,1.07,2.91c0,2.1,0,3.79,0,4.31s.28.91,1.08.76A15.73,15.73,0,0,0,16,.29Z"/>
			</svg>
			GitHub
		</a>
	</li>
	<li>
		<a href="https://opencollective.com/romejs">
			<svg data-name="open collective" class="open-collective icon" height="32px" width="32px"	xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
				<path class="path-1" d="M12,19.57A7.57,7.57,0,0,1,12,4.43a7.34,7.34,0,0,1,4.1,1.23l3-3A11.9,11.9,0,0,0,12,.27a11.74,11.74,0,1,0,0,23.47,11.56,11.56,0,0,0,7.12-2.44L16,18.22a6.25,6.25,0,0,1-4,1.35Z"/>
				<path class="path-2" d="M19.54,12a7.64,7.64,0,0,1-1.22,4.1l3.07,3.08a11.86,11.86,0,0,0,2.38-7.12A11.48,11.48,0,0,0,21.33,5l-3,3a7.26,7.26,0,0,1,1.22,4Z"/>
			</svg>
		Open Collective
		</a>
	</li>
	<li>
		<a href="https://twitter.com/romejsdev">
			<svg data-name="twitter" class="twitter icon"	xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 31.21">
				<path class="path-1" d="M10.21,28.28c11.77,0,18.21-9.75,18.21-18.21,0-.28,0-.55,0-.83a12.75,12.75,0,0,0,3.2-3.31,12.74,12.74,0,0,1-3.68,1A6.37,6.37,0,0,0,30.73,3.4,12.84,12.84,0,0,1,26.67,5a6.41,6.41,0,0,0-10.91,5.84A18.15,18.15,0,0,1,2.58,4.1a6.39,6.39,0,0,0,2,8.54,6.32,6.32,0,0,1-2.91-.8v.08A6.41,6.41,0,0,0,6.79,18.2a6.42,6.42,0,0,1-2.89.11,6.4,6.4,0,0,0,6,4.44,12.89,12.89,0,0,1-8,2.75A11.23,11.23,0,0,1,.4,25.4a18.09,18.09,0,0,0,9.81,2.87"/>
		</svg>
		Twitter</a>
	</li>
</ul>
