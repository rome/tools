---
title: Lint Rule css/noDuplicateCustomProperties
layout: layouts/rule.liquid
showHero: false
description: prevent custom properties with the same name inside a block
stylelint-rule: https://stylelint.io/user-guide/rules/declaration-block-no-duplicate-custom-properties
eleventyNavigation:
	key: lint-rules/css/noDuplicateCustomProperties
	parent: lint-rules
	title: css/noDuplicateCustomProperties
---

# css/noDuplicateCustomProperties

Prevent the usage of custom proprieties (CSS vars) that have the same name inside the same
CSS block.

<!-- GENERATED:START(hash:0a21fbc28a057dfc7858203e46b2447461ff25b6,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**stylelint Equivalent:** [declaration-block-no-duplicate-custom-properties](https://stylelint.io/user-guide/rules/declaration-block-no-duplicate-custom-properties)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:d35fddd88d905fb04a38b71d9dabd02a775530db,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-css"><code class="language-css">.<span class="token string">style</span> <span class="token punctuation">{</span>
	<span class="token property">--custom-prop</span><span class="token punctuation">:</span> <span class="token string">foo</span><span class="token punctuation">;</span>
	<span class="token property">--custom-prop</span><span class="token punctuation">:</span> <span class="token string">bar</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-css"><code class="language-css">.<span class="token string">style</span> <span class="token punctuation">{</span>
	<span class="token property">--custom-prop</span><span class="token punctuation">:</span> <span class="token string">foo</span><span class="token punctuation">;</span>
	<span class="token property">--custom-PROP</span><span class="token punctuation">:</span> <span class="token string">bar</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
