---
title: Lint Rule js/emptyBlocks
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/emptyBlocks
	parent: lint-rules
	title: js/emptyBlocks
---

# js/emptyBlocks

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:70faf58a6208a1aea90b15b8c2b6145fb48ac72a,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:9</span> <strong>lint/js/emptyBlocks</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>empty logic blocks</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty logic blocks usually result from incomplete refactoring.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">// foo;</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:7</span> <strong>lint/js/emptyBlocks</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>empty logic blocks</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token comment">// foo;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty logic blocks usually result from incomplete refactoring.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token function">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">// empty</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
