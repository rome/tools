---
title: Lint Rule js/noGetterReturn
layout: layouts/rule.liquid
description: enforce `return` statements in getters
eslint-rule: https://eslint.org/docs/rules/getter-return
eleventyNavigation:
	key: lint-rules/js/noGetterReturn
	parent: lint-rules
	title: js/noGetterReturn
---

# js/noGetterReturn

<!-- GENERATED:START(hash:aaa96530665f250abcad32cfc5d3bb343ec8fc5d,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce `return` statements in getters

**ESLint Equivalent:** [getter-return](https://eslint.org/docs/rules/getter-return)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:268b350a5f6b2af5cd286a410cc2c11160225ee3,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">p</span> <span class="token punctuation">{</span>
	<span class="token variable">get</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:12</span> <strong>lint/js/noGetterReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Return a value at the end of a getter method</strong></span><span style="color: Tomato;"> instead of empty block.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">p</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token variable">get</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Getters that do not return values are either typos or should not be</span>
    <span style="color: DodgerBlue;">getters.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">p</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
	<span class="token variable">get</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:12</span> <strong>lint/js/noGetterReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Return a value at the end of a getter method</strong></span><span style="color: Tomato;"> instead of empty block.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">p</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token variable">get</span> <span class="token variable">name</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>  <span class="token punctuation">}</span><span class="token punctuation">,</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Getters that do not return values are either typos or should not be</span>
    <span style="color: DodgerBlue;">getters.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">p</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token variable">Object</span><span class="token punctuation">.</span><span class="token variable">defineProperty</span><span class="token punctuation">(</span><span class="token variable">p</span><span class="token punctuation">,</span> <span class="token punctuation">{</span>
	<span class="token variable">get</span><span class="token punctuation">:</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
