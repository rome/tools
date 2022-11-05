---
title: Introducing Rome
description: A linter for JavaScript and TypeScript. The first in a suite of tools for bundling, compiling, testing, and more, for frontend languages.
author_name: Sebastian McKenzie
author_url: https://twitter.com/sebmck
author_avatar: /img/blog/sebmck-avatar.jpg
date: 2020-08-09
tags:
	- release
	- post
permalink: /blog/2020/08/08/introducing-rome.html
layout: layouts/blog.liquid
---

We're excited to announce the first beta release and general availability of the **Rome** linter for JavaScript and TypeScript.

This is the beginning of an entire suite of tools. Rome is not only linter, but also a compiler, bundler, test runner, and [more](/#development-status), for JavaScript, TypeScript, HTML, JSON, Markdown, and CSS. We aim to unify the entire frontend development toolchain.

<!-- DESCRIPTION_END -->

Rome is a [monolithic](https://en.wikipedia.org/wiki/Monolithic_application) tool containing functionality that has traditionally been separate tools in the frontend ecosystem. We call this a toolchain. It is **not** a collection of existing tools, but completely custom, built [largely](/docs/credits) from scratch, contained entirely in a single package and codebase.

Rome is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

We have significant implementations already for most components, including compiling, bundling, and testing, all using the same core abstractions and internal libraries. Rome is even [self-hosted](https://en.wikipedia.org/wiki/Self-hosting_(compilers)) and we use them to [build and develop Rome itself](https://github.com/rome/tools/blob/main/CONTRIBUTING.md)! Once usage as a linter has stabilized, we will move onto the unreleased features to prepare and generalize them for public usage.

Linting is an excellent low-risk way for us to mature and validate shared functionality like editor integration, configuration, parallelization, caching, parsing, and dependency analysis. It is not in the critical build path for development so is relatively straight forward and safe to adopt.

{% include docs/cli-screenshots/check.md %}

Even though this is a beta, we have support for over 100 lint rules, including the most common rules needed when working with TypeScript and React. See the full [list of rules](/docs/lint/rules). You can expect a full featured experience, however may still encounter bugs. With most of the feature development out of the way we will be focusing primarily on stability.

Most of our rules have been inspired by their ESLint equivalents. This required a complete rewrite to utilize Rome's API that differ extensively from existing tools. Without the amazing work of the ESLint team and community establishing these recommended patterns, we would not have been able to pull this off. Refer to the [lint rule docs](/docs/lint/rules) for direct references.

Since open sourcing, at the beginning of the year, we have received contributions from over [70 contributors](https://github.com/rome/tools/graphs/contributors) and [600 pull requests](https://github.com/rome/tools/pulls?q=is%3Apr+is%3Amerged). We have an established [team](/about#team), and a [code of conduct](https://github.com/rome/tools/blob/main/CODE_OF_CONDUCT.md) enforcement policy. This ensures transparency around project decisions, moderation, and direction.

You can read more about Rome, including how to get started, in our [documentation](/). If you would like to get involved, check out our [contributing instructions](https://github.com/rome/tools/blob/main/CONTRIBUTING.md). If you are interested in reading more about the history and rationale for the project, you can continue reading below.

{% include action-links.liquid %}

## History

I created **6to5** in 2014 (now called [Babel](https://babeljs.io/)), a JavaScript transpiler that compiled new ES6 code to ES5 that could be run in any browser. At the time I wasn't trying to solve any particular problem, I was just building and experimenting with existing libraries and teaching myself new concepts. Over time though, as the project exploded in popularity, so did the scope and mission of the project.

We renamed **6to5** to **Babel** to take on a new role as a general platform for doing static JavaScript transformation. This meant a plugin system and a new commitment to supporting future JavaScript standards and proposed features.

We also planned on expanding Babel to be used as the base for other JavaScript language tooling by using shared public libraries. From the Babel blog post "[6.0.0 Released](https://babeljs.io/blog/2015/10/29/6.0.0)" by [Jamie Kyle](https://twitter.com/buildsghost):

> But we think we can go even further. Babel should be able to power minifiers, linters, formatters, syntax highlighters, code completion tools, type checkers, codemod tools, and every other tool to be using the same foundation to do their job better than ever before.

{% include figure.liquid, width: 600, src: "/img/blog/6to5-to-babel.png", description: "Old 6to5 logo with an arrow pointing to the new Babel logo" %}

In 2016, I left the project, and those plans never materialized. Over time, I learned the Babel would not have been able to successfully adapt to execute on this vision anyway. The solution to plugins was "expose all the internals" which is an extremely large API surface area to maintain and restricts your ability to make any changes.

Making the necessary modifications to Babel to allow for it to be a reliable base for other tools would have required changes to absolutely everything. The architecture is bound to the initial design choices I made in 2014 when I was learning about parsers, ASTs, and compilers.

There would have been no way to provide backward compatibility, and with expectations already set, any significant changes to the project would have introduced an extremely high amount of ecosystem churn.

In an ode to the [Ship of Theseus](https://en.wikipedia.org/wiki/Ship_of_Theseus), if all components of an object are replaced, is it the same object? There is far less confusion and friction to release something entirely new than drastically change something that's already in widespread usage. I had since moved on from the project, and so any evolution would not have been possible, and would have required complete vision alignment.

Even though I wasn't involved in Babel, I still maintained a presence in the developer tooling ecosystem. I eventually went on to develop other tools like [Yarn](https://yarnpkg.com/) and be involved in projects like [Prepack](https://prepack.io/) and [Flipper](https://fbflipper.com/). This work continued to develop the ideas that would eventually become Rome and shaped my philosophy on developer experience. A focus on excellent errors, clean user interface, and minimal configuration.

{% include figure.liquid, width: 500, src: "/img/blog/introducing-rome-post-babel-logos.png", description: "The logos of Yarn, Prepack, and Flipper" %}

The original idea behind Babel had always stuck. What sort of capabilities would be possible if a tool did more than one thing? This sort of philosophy has always seemed alien in the JavaScript ecosystem where micropackages and arbitrary separation of concerns are the norms. These tools being independent has never provided the sort of user autonomy and efficiency that most skeptics of [monolithic](https://en.wikipedia.org/wiki/Monolithic_application) tools proclaim.

Language tooling maintainers spend so much time working on the same things. Processing source code, whether it's in a transpiler like Babel, linting it in ESLint, or bundling it in webpack, is fundamentally the same problem with overlapping responsibilities and technical implementation.

A linter in the JavaScript ecosystem is exactly the same as a compiler. They both consume code, then produce compiled code and errors. What the compiled code is varies. In the case of a linter, the compiled code is formatted source code with automatic fixes applied. The more powerful your compilation infrastructure, the more theoretically powerful your linter could be. These commonalities extend to pretty much anything that involves language processing. We can build more powerful tools using a shared base that is easily adaptable.

I had always maintained private repos where I would experiment with ideas, but the original code that would become Rome was started at the beginning of 2017 while I was working for Facebook in my free time. I continued iterating and experimenting on the idea and building out all the different pieces, until 2019, when I was given the opportunity to work on and explore using it at Facebook full-time.

I eventually left Facebook and the project became open source in February 2020 where I would continue it as an independent community-driven project.

Rome is the spiritual successor of Babel. I've taken the lessons I learnt, and refined the mission. Rather than exposing a large public API for other tools to be built on, we are building them all in one place, with batteries included. I'm excited to be trying something new that the JavaScript and web ecosystem haven't seen before.

&mdash; [Sebastian McKenzie](https://twitter.com/sebmck)
