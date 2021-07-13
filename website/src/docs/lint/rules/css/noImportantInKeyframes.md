---
title: Lint Rule css/noImportantInKeyframes
layout: layouts/rule.liquid
description: disallow `!important` in `@keyframe` rules
stylelint-rule: https://stylelint.io/user-guide/rules/keyframe-declaration-no-important
eleventyNavigation:
	key: lint-rules/css/noImportantInKeyframes
	parent: lint-rules
	title: css/noImportantInKeyframes
---

# css/noImportantInKeyframes

Disallow `!important` in `@keyframe` rules.

<!-- GENERATED:START(hash:0094121294949d6adeae81bbbd4799e0eb090fce,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**stylelint Equivalent:** [keyframe-declaration-no-important](https://stylelint.io/user-guide/rules/keyframe-declaration-no-important)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f4895cfe97c501a12a772cb97009a7e815fb7375,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token atrule">@keyframes</span> <span class="token string">foo</span> <span class="token punctuation">{</span>
  <span class="token string">from</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
    <span class="token property">width</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>

  <span class="token string">to</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">1</span> !<span class="token string">important</span><span class="token punctuation">;</span>
    <span class="token property">width</span><span class="token punctuation">:</span> 100px !<span class="token string">important</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token atrule">@keyframes</span> <span class="token string">foo</span> <span class="token punctuation">{</span>
  <span class="token string">from</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>

  <span class="token string">to</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
