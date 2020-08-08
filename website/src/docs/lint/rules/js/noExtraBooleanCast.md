---
title: Lint Rule js/noExtraBooleanCast
layout: layouts/rule.liquid
description: disallow unnecessary boolean casts
eslint-rule: https://eslint.org/docs/rules/no-extra-boolean-cast
eleventyNavigation:
	key: lint-rules/js/noExtraBooleanCast
	parent: lint-rules
	title: js/noExtraBooleanCast
---

# js/noExtraBooleanCast

<!-- GENERATED:START(hash:5e912205fd815fbed9657dce1be77e8cef465d98,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow unnecessary boolean casts

**ESLint Equivalent:** [no-extra-boolean-cast](https://eslint.org/docs/rules/no-extra-boolean-cast)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:1a975a50081af0e7bcb9023aef0b2534f94398cf,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noExtraBooleanCast</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is not necessary to use double-negation when a value will already</span>
    <span style="color: DodgerBlue;">be coerced to a boolean.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:7</span> <strong>lint/js/noExtraBooleanCast</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is not necessary to use double-negation when a value will already</span>
    <span style="color: DodgerBlue;">be coerced to a boolean.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>
<span class="token keyword">do</span> <span class="token punctuation">{</span>
	<span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:9</span> <strong>lint/js/noExtraBooleanCast</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong><span class="token keyword">do</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>  <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is not necessary to use double-negation when a value will already</span>
    <span style="color: DodgerBlue;">be coerced to a boolean.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token function">foo</span><span class="token punctuation">;</span> <span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:7</span> <strong>lint/js/noExtraBooleanCast</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token function">foo</span><span class="token punctuation">;</span> <span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It is not necessary to use double-negation when a value will already</span>
    <span style="color: DodgerBlue;">be coerced to a boolean.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
