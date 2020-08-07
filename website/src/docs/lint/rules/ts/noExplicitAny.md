---
title: Lint Rule ts/noExplicitAny
layout: layouts/rule.liquid
description: it bans the use of `any`
eleventyNavigation:
	key: lint-rules/ts/noExplicitAny
	parent: lint-rules
	title: ts/noExplicitAny
---

# ts/noExplicitAny

Using the `any` type defeats the purpose of using TypeScript. 

When `any` is used, all compiler type checks around that value are ignored.
