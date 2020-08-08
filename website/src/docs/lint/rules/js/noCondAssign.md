---
title: Lint Rule js/noCondAssign
layout: layouts/rule.liquid
description: disallow assignment operators in conditional expressions
eslint-rule: https://eslint.org/docs/rules/no-cond-assign
eleventyNavigation:
	key: lint-rules/js/noCondAssign
	parent: lint-rules
	title: js/noCondAssign
---

# js/noCondAssign

<!-- GENERATED:START(hash:48ce22b6b3962346ae19f3e6f8adf5e6fc44da85,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow assignment operators in conditional expressions

**ESLint Equivalent:** [no-cond-assign](https://eslint.org/docs/rules/no-cond-assign)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:ba881b16eb706d833139ed7598785af7b1cd001d,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">let</span> <span class="token function">i</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span> <span class="token function">i</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token function">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:16</span> <strong>lint/js/noCondAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not assign </span><span style="color: Tomato;"><strong>variables in loop conditions</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">let</span> <span class="token function">i</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span> <span class="token function">i</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token function">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is a common typo to mistype an equality operator as an assignment</span>
    <span style="color: DodgerBlue;">operator.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noCondAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not assign </span><span style="color: Tomato;"><strong>variables in loop conditions</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is a common typo to mistype an equality operator as an assignment</span>
    <span style="color: DodgerBlue;">operator.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:7</span> <strong>lint/js/noCondAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not assign </span><span style="color: Tomato;"><strong>variables in loop conditions</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is a common typo to mistype an equality operator as an assignment</span>
    <span style="color: DodgerBlue;">operator.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">do</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:9</span> <strong>lint/js/noCondAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not assign </span><span style="color: Tomato;"><strong>variables in loop conditions</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">do</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">)</span>
     <strong> │ </strong>         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is a common typo to mistype an equality operator as an assignment</span>
    <span style="color: DodgerBlue;">operator.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span> <span class="token punctuation">?</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noCondAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not assign </span><span style="color: Tomato;"><strong>variables in loop conditions</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">(</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span> <span class="token punctuation">?</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is a common typo to mistype an equality operator as an assignment</span>
    <span style="color: DodgerBlue;">operator.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token function">foo</span> <span class="token operator">=</span> <span class="token function">foo</span><span class="token punctuation">.</span><span class="token function">bar</span><span class="token punctuation">)</span> <span class="token operator">!==</span> <span class="token function">undefined</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token operator">++</span> <span class="token operator">===</span> <span class="token number">3</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span> <span class="token punctuation">?</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
