---
title: Lint Rule js/noImportAssign
layout: layouts/rule.liquid
description: disallow assigning to imported bindings
eslint-rule: https://eslint.org/docs/rules/no-import-assign
eleventyNavigation:
	key: lint-rules/js/noImportAssign
	parent: lint-rules
	title: js/noImportAssign
---

# js/noImportAssign

<!-- GENERATED:START(hash:dbff558a6a3f722c7582c4a0f9352a18982269bf,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow assigning to imported bindings

**ESLint Equivalent:** [no-import-assign](https://eslint.org/docs/rules/no-import-assign)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:1dbd753563a2598640f18eb29d6e69b2b9d68431,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token punctuation">[</span><span class="token variable">x</span><span class="token punctuation">]</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:1</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token punctuation">[</span><span class="token variable">x</span><span class="token punctuation">]</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong> <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">}</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">}</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token operator">++</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token operator">++</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token punctuation">[</span><span class="token operator">...</span><span class="token variable">x</span><span class="token punctuation">]</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:4</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token punctuation">[</span><span class="token operator">...</span><span class="token variable">x</span><span class="token punctuation">]</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">x</span><span class="token punctuation">}</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:5</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">x</span><span class="token punctuation">}</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>     <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token variable">x</span> <span class="token keyword">in</span> <span class="token variable">y</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:5</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token variable">x</span> <span class="token keyword">in</span> <span class="token variable">y</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>     <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token operator">+=</span><span class="token number">1</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token operator">+=</span><span class="token number">1</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token operator">*</span> <span class="token variable">as</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token operator">*</span> <span class="token variable">as</span> <span class="token variable">x</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noImportAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;y&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token operator">=</span><span class="token number">1</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead of reassigning an import.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
