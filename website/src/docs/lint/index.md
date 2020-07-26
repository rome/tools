---
title: Linting
layout: layouts/page.liquid
eleventyNavigation: {
	key: linting,
	title: Linting
}
---

# Linting

<ul class="action-buttons">
	<li class="label">Quick Links:</li>
	<li><a href="/docs/cli">CLI</a></li>
	<li><a href="/docs/project-config">Configuration</a></li>
	<li><a href="/docs/lint/rules">Rules</a></li>
</ul>

We've built Rome to be fantastic at displaying [diagnostics](/docs/diagnostics) and providing as much information as possible for you to understand and fix them. We don't believe that existing JavaScript linters do enough. More often they get in the way. Sometimes conventions, while making code more consistent, make it difficult to remember and work on. We have tried to address this in Rome with the following ways:

 - Providing as much context as possible on why a diagnostic was produced.
 - Make it obvious and tell you how to fix it (if possible).
 - Offer powerful autofixes so you don't even need to make most changes yourself (via [fixes]()).
 - Offer autofix suggestions for scenarios for potentially unsafe fixes (via [suggestions]()).
 - Make it easy to review all diagnostics and perform actions on them (via [review]()).

To use the Rome linter, you also need to use the autoformatter. Our autofixes rely on being able to modify your code in a rich way which requires being able to consistently format the result.

## Usage

Linting is done by running the [`rome check`](/docs/cli/commands/check) command.

```bash
rome check
```

## Formatting



## Fixing
