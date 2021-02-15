---
title: Lint Rule js/noRestrictedGlobals
layout: layouts/rule.liquid
description: disallow certain global variables
eslint-rule: https://eslint.org/docs/rules/no-restricted-globals
eleventyNavigation:
	key: lint-rules/js/noRestrictedGlobals
	parent: lint-rules
	title: js/restrictedGlobals
---

# js/noRestrictedGlobals

<!-- GENERATED:START(hash:481826b79c63d94e31193a209e6081ec995ae582,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow certain global variables

**ESLint Equivalent:** [no-restricted-globals](https://eslint.org/docs/rules/no-restricted-globals)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:8ee7a8386c6bd4be5400173145ad167c7f0950d8,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">event</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:12</span> <strong>lint/js/noRestrictedGlobals</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use the global variable </span><span style="color: Tomato;"><strong>event</strong></span><span style="color: Tomato;">.</span>

    <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">event</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">event</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noRestrictedGlobals</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use the global variable </span><span style="color: Tomato;"><strong>event</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">event</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">info</span><span class="token punctuation">(</span><span class="token variable">event</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
