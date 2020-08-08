---
title: Lint Rule js/noUnsafeFinally
layout: layouts/rule.liquid
description: disallow control flow statements in `finally` blocks
eslint-rule: https://eslint.org/docs/rules/no-unsafe-finally
eleventyNavigation:
	key: lint-rules/js/noUnsafeFinally
	parent: lint-rules
	title: js/noUnsafeFinally
---

# js/noUnsafeFinally

<!-- GENERATED:START(hash:14db8e694f7ca033dc3fc28e3669f576bcc7be68,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow control flow statements in `finally` blocks

**ESLint Equivalent:** [no-unsafe-finally](https://eslint.org/docs/rules/no-unsafe-finally)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:6b5d47ff602c62f3547f7bf4fc5fea0863233e04,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">greet1</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">try</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Try&apos;</span><span class="token punctuation">)</span>
	<span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">err</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> <span class="token number">1</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:7:2</span> <strong>lint/js/noUnsafeFinally</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>JSReturnStatement</strong></span><span style="color: Tomato;"> inside a </span><span style="color: Tomato;"><strong>finally</strong></span><span style="color: Tomato;"> clause is unsafe.</span>

  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    <span class="token keyword">return</span> <span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  9</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Do not use control flow statements inside finally clauses.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">greet2</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">try</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Try&apos;</span><span class="token punctuation">)</span>
	<span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">err</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:7:2</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">No loop label found</span>

  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  9</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">greet3</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">try</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Try&apos;</span><span class="token punctuation">)</span>
	<span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">err</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
		<span class="token keyword">continue</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:7:2</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">No loop label found</span>

  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    <span class="token keyword">continue</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  9</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">greet4</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">try</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Try&apos;</span><span class="token punctuation">)</span>
	<span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">err</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
		<span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Finally&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:7:2</span> <strong>lint/js/noUnsafeFinally</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>JSThrowStatement</strong></span><span style="color: Tomato;"> inside a </span><span style="color: Tomato;"><strong>finally</strong></span><span style="color: Tomato;"> clause is unsafe.</span>

  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">throw</span> <span class="token variable">err</span><span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span> <span class="token keyword">finally</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    <span class="token keyword">throw</span> <span class="token keyword">new</span> <span class="token variable">Error</span><span class="token punctuation">(</span><span class="token string">&apos;Finally&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  9</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Do not use control flow statements inside finally clauses.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
