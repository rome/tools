---
title: Lint Rule js/noEmptyBlocks
layout: layouts/rule.liquid
description: disallow empty block statements
eslint-rule: https://eslint.org/docs/rules/no-empty
eleventyNavigation:
	key: lint-rules/js/noEmptyBlocks
	parent: lint-rules
	title: js/noEmptyBlocks
---

# js/noEmptyBlocks

<!-- GENERATED:START(hash:efb273c000cc876ea875410d0e3790f32f0ecebe,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow empty block statements

**ESLint Equivalent:** [no-empty](https://eslint.org/docs/rules/no-empty)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:280c4f60d0feb0c28d91ee10174ef38605c1d5cb,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:9</span> <strong>lint/js/noEmptyBlocks</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>empty logic blocks</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Empty logic blocks usually result from incomplete refactoring.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">// foo;</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:7</span> <strong>lint/js/noEmptyBlocks</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>empty logic blocks</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token comment">// foo;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Empty logic blocks usually result from incomplete refactoring.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">// empty</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
