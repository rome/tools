---
title: Lint Rule js/noFunctionAssign
layout: layouts/rule.liquid
description: disallow reassigning `function` declarations
eslint-rule: https://eslint.org/docs/rules/no-func-assign
eleventyNavigation:
	key: lint-rules/js/noFunctionAssign
	parent: lint-rules
	title: js/noFunctionAssign
---

# js/noFunctionAssign

<!-- GENERATED:START(hash:d265996496d701ebc4fd414cb36463aad523cd47,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow reassigning `function` declarations

**ESLint Equivalent:** [no-func-assign](https://eslint.org/docs/rules/no-func-assign)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:950ba561a1e677646a6978bb1702610b56b529d4,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:19</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span>
                       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:17</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">[</span><span class="token variable">foo</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">[</span><span class="token variable">foo</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:5</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">[</span><span class="token variable">foo</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">[</span><span class="token variable">foo</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:19</span> <strong>lint/js/noFunctionAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign a function declaration</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">var</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">var</span> <span class="token variable">foo</span><span class="token punctuation">;</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">var</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">var</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token keyword">function</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">var</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token keyword">function</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">bar</span> <span class="token keyword">from</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">var</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
