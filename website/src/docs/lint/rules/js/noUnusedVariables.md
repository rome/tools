---
title: Lint Rule js/noUnusedVariables
layout: layouts/rule.liquid
description: disallow unused variables
eslint-rule: https://eslint.org/docs/rules/no-unused-vars
eleventyNavigation:
	key: lint-rules/js/noUnusedVariables
	parent: lint-rules
	title: js/noUnusedVariables
---

# js/noUnusedVariables

<!-- GENERATED:START(hash:4ab38cdce1ed7ab05a6f370dccf8050b3092c442,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow unused variables

**ESLint Equivalent:** [no-unused-vars](https://eslint.org/docs/rules/no-unused-vars)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:261161e50a68ab30e274a5b69ba8ee8cce09707f,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token number">4</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:6</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The const variable </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> is unused.</span>

    <span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token number">4</span><span class="token punctuation">;</span>
          <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token number">4</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The let variable </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> is unused.</span>

    <span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token number">4</span><span class="token punctuation">;</span>
        <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">myVar</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The parameter variable </span><span style="color: Tomato;"><strong>myVar</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">myVar</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:9</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The function variable </span><span style="color: Tomato;"><strong>foo</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">myVar</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span>
<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The parameter variable </span><span style="color: Tomato;"><strong>myVar</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">myVar</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">b</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The parameter variable </span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">b</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:6</span> <strong>lint/js/noUnusedVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The const variable </span><span style="color: Tomato;"><strong>foo</strong></span><span style="color: Tomato;"> is unused.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Unused variables are dead code and usually the result of incomplete</span>
    <span style="color: DodgerBlue;">refactoring.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">_</span><span class="token punctuation">,</span> <span class="token variable">b</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">b</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">_</span><span class="token punctuation">,</span> <span class="token variable">b</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">b</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
