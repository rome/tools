---
title: Lint Rule js/noSetterReturn
layout: layouts/rule.liquid
description: disallow returning values from setters
eslint-rule: https://eslint.org/docs/rules/no-setter-return
eleventyNavigation:
	key: lint-rules/js/noSetterReturn
	parent: lint-rules
	title: js/noSetterReturn
---

# js/noSetterReturn

<!-- GENERATED:START(hash:aa38e3c37e0fec6b7a2dd86f00f8f5a2618c59cb,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow returning values from setters

**ESLint Equivalent:** [no-setter-return](https://eslint.org/docs/rules/no-setter-return)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:c664a625e1a9f3b3df51b425520bca803b7e08b9,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">p</span> <span class="token punctuation">{</span>
	<span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:3</span> <strong>lint/js/noSetterReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>return a value</strong></span><span style="color: Tomato;"> at the end of a </span><span style="color: Tomato;"><strong>setter method</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">}</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Setters that return values are either typos or should not be setters.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">p</span> <span class="token punctuation">{</span>
	<span class="token variable">static</span> <span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:3</span> <strong>lint/js/noSetterReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>return a value</strong></span><span style="color: Tomato;"> at the end of a </span><span style="color: Tomato;"><strong>setter method</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">static</span> <span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">}</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Setters that return values are either typos or should not be setters.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">p</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
	<span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:3</span> <strong>lint/js/noSetterReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>return a value</strong></span><span style="color: Tomato;"> at the end of a </span><span style="color: Tomato;"><strong>setter method</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token keyword">return</span> <span class="token string">&apos;wrong&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">}</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Setters that return values are either typos or should not be setters.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">p</span> <span class="token punctuation">{</span>
	<span class="token variable">set</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">value</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
