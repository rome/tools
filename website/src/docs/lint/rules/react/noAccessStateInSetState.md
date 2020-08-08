---
title: Lint Rule react/noAccessStateInSetState
layout: layouts/rule.liquid
description: prevent using `this.state` within a `this.setState`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-access-state-in-setstate.md
eleventyNavigation:
	key: lint-rules/react/noAccessStateInSetState
	parent: lint-rules
	title: react/noAccessStateInSetState
---

# react/noAccessStateInSetState

<!-- GENERATED:START(hash:fc6d0ed1c8dc167c1a09808d00e056ee841d2059,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent using `this.state` within a `this.setState`

**ESLint Equivalent:** [no-access-state-in-setstate](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-access-state-in-setstate.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:777ad53bbdc7ec8c24057109863b28c085b2f788,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token keyword">function</span> <span class="token variable">increment</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:10</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unknown class property start</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">function</span> <span class="token variable">increment</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token keyword">function</span> <span class="token variable">increment</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">value</span><span class="token punctuation">:</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:10</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unknown class property start</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">function</span> <span class="token variable">increment</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">value</span><span class="token punctuation">:</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token keyword">function</span> <span class="token variable">toggle</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token operator">!</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:10</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unknown class property start</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">function</span> <span class="token variable">toggle</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
  <strong>  4</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token operator">!</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token keyword">function</span> <span class="token variable">toggle</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:10</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unknown class property start</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">function</span> <span class="token variable">toggle</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
  <strong>  4</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noAccessStateInSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> within a </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> call.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token number">1</span> <span class="token operator">+</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Batched state calls could result in unexpected errors due to stale</span>
    <span style="color: DodgerBlue;">state data.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span> <span class="token operator">+</span> <span class="token number">1</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noAccessStateInSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> within a </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> call.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span> <span class="token operator">+</span> <span class="token number">1</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Batched state calls could result in unexpected errors due to stale</span>
    <span style="color: DodgerBlue;">state data.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noAccessStateInSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> within a </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> call.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  7</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Batched state calls could result in unexpected errors due to stale</span>
    <span style="color: DodgerBlue;">state data.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
			<span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noAccessStateInSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> within a </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> call.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">,</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token variable">value</span><span class="token punctuation">:</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">value</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Batched state calls could result in unexpected errors due to stale</span>
    <span style="color: DodgerBlue;">state data.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">update</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">bar</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">increment</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token variable">prevState</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">value</span><span class="token punctuation">:</span> <span class="token variable">prevState</span><span class="token punctuation">.</span><span class="token variable">value</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
