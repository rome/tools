---
title: Lint Rule css/noInvalidGridTemplateAreas
layout: layouts/rule.liquid
showHero: false
description: checks that `grid-template-areas` have valid names
stylelint-rule: https://stylelint.io/user-guide/rules/named-grid-areas-no-invalid
eleventyNavigation:
	key: lint-rules/css/noInvalidGridTemplateAreas
	parent: lint-rules
	title: css/noInvalidGridTemplateAreas
---

# css/noInvalidGridTemplateAreas

Checks that `grid-template-areas` have valid names. In the specifics, area templates
needs to have the same amount of cell tokens.

<!-- GENERATED:START(hash:407bb4d2e531b0becdce7d9332f3399eec50284b,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**stylelint Equivalent:** [named-grid-areas-no-invalid](https://stylelint.io/user-guide/rules/named-grid-areas-no-invalid)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:a792750edde5d56e6ed73954a16433ac9cf3540a,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> &quot;a a a&quot;
												 &quot;b b&quot;<span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.css:2:13</span> <strong>lint/css/noInvalidGridTemplateAreas</strong> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">All strings must have the same number of cell tokens</span>

  <strong>  1</strong><strong> │ </strong><span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> &quot;a a a&quot;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>                         &quot;b b&quot;<span class="token punctuation">;</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-css"><code class="language-css"><span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> &quot;a&quot; &quot;b b&quot;<span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.css:1:29</span> <strong>lint/css/noInvalidGridTemplateAreas</strong> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">All strings must have the same number of cell tokens</span>

    <span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> &quot;a&quot; &quot;b b&quot;<span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> &quot;a a a&quot;
												 &quot;b b b&quot;<span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-css"><code class="language-css"><span class="token string">a</span> <span class="token punctuation">{</span> <span class="token property">grid-template-areas</span><span class="token punctuation">:</span> <span class="token string">none</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
