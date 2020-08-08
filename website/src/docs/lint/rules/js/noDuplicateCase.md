---
title: Lint Rule js/noDuplicateCase
layout: layouts/rule.liquid
description: disallow duplicate case labels
eslint-rule: https://eslint.org/docs/rules/no-duplicate-case
eleventyNavigation:
	key: lint-rules/js/noDuplicateCase
	parent: lint-rules
	title: js/noDuplicateCase
---

# js/noDuplicateCase

<!-- GENERATED:START(hash:1cad8cd63a412e8a4bd0ceed75e6888dc7e941c5,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow duplicate case labels

**ESLint Equivalent:** [no-duplicate-case](https://eslint.org/docs/rules/no-duplicate-case)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:19b5bdc5f66d87eba7033014ffab88e26fda1592,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;b&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;c&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;d&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;c&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:7:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>&quot;c&quot;</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

   <strong>  9</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token string">&apos;d&apos;</span><span class="token punctuation">:</span>
  <strong>  10</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 11</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token string">&apos;c&apos;</span><span class="token punctuation">:</span>
      <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  12</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  13</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token number">3</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token number">1</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">3</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:5:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>2</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

   <strong>  7</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token number">3</span><span class="token punctuation">:</span>
   <strong>  8</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
   <strong><span style="color: Tomato;">&gt;</span></strong><strong> 9</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token number">2</span><span class="token punctuation">:</span>
      <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  10</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  11</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token number">3</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token number">1</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2n</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">3</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2n</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:5:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>2n</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

   <strong>  7</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token number">3</span><span class="token punctuation">:</span>
   <strong>  8</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
   <strong><span style="color: Tomato;">&gt;</span></strong><strong> 9</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token number">2n</span><span class="token punctuation">:</span>
      <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  10</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  11</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token variable">foo</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token variable">foo</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>&quot;a&quot;</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token variable">foo</span><span class="token punctuation">:</span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token variable">foo</span><span class="token punctuation">:</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  7</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>&quot;foo&quot;</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">:</span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">:</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  7</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token boolean">null</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token boolean">null</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:6</span> <strong>lint/js/noDuplicateCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not duplicate the </span><span style="color: Tomato;"><strong>null</strong></span><span style="color: Tomato;"> case.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Duplicated switch logic paths are hard to follow and usually typos.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token boolean">null</span><span class="token punctuation">:</span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token boolean">null</span><span class="token punctuation">:</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>    <span class="token keyword">break</span><span class="token punctuation">;</span>
  <strong>  7</strong><strong> │ </strong>  <span class="token keyword">default</span><span class="token punctuation">:</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token string">&apos;a&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;b&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;c&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;d&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token number">3</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token number">1</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">3</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">2n</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">expr</span> <span class="token operator">=</span> <span class="token number">3</span><span class="token punctuation">;</span>
<span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token variable">expr</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token number">1</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;\n1&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token number">1n</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;null&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token boolean">null</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token variable">foo</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">case</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
	<span class="token keyword">default</span><span class="token punctuation">:</span>
		<span class="token keyword">break</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
