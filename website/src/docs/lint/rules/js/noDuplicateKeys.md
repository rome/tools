---
title: Lint Rule js/noDuplicateKeys
layout: layouts/rule.liquid
description: disallow duplicate keys in object literals
eslint-rule: https://eslint.org/docs/rules/no-dupe-keys
eleventyNavigation:
	key: lint-rules/js/noDuplicateKeys
	parent: lint-rules
	title: js/noDuplicateKeys
---

# js/noDuplicateKeys

<!-- GENERATED:START(hash:1b69c57fd50f9c650f9cdbd1779e40a6c5960939,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow duplicate keys in object literals

**ESLint Equivalent:** [no-dupe-keys](https://eslint.org/docs/rules/no-dupe-keys)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:d420d85752497fe667e0cd3c33079b23cf4665d7,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
	<span class="token variable">test</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
	<span class="token variable">test2</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
	<span class="token variable">test</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:1</span> <strong>lint/js/noDuplicateKeys</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate component key. Check the </span><span style="color: Tomato;"><strong>test</strong></span><span style="color: Tomato;"> key.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">test</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
  <strong>  3</strong><strong> │ </strong>  <span class="token variable">test2</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>  <span class="token variable">test</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">;</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
