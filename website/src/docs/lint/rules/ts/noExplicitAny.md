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

<!-- GENERATED:START(hash:a4334779b05ba77f60a80fea3e43c24c4ad27e31,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
it bans the use of `any`
<!-- GENERATED:END(id:description) -->
