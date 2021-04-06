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

<!-- GENERATED:START(hash:dfb24e0bc37deabb8a3c545dbf97ee7da490bc61,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Boolean(</strong></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>)</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">Boolean(foo)</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">Boolean(foo)</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:6</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Boolean(</strong></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>)</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:5</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Boolean(</strong></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>)</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:7</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">foo</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">let</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>
<span class="token keyword">do</span> <span class="token punctuation">{</span>
	<span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:9</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong><span class="token keyword">do</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>  <span class="token number">1</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Boolean(</strong></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><strong>)</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token variable">foo</span><span class="token punctuation">;</span> <span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:7</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token variable">foo</span><span class="token punctuation">;</span> <span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">foo</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">new</span> <span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:12</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">new</span> <span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">x</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token operator">!</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token operator">!</span><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">x</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noExtraBooleanCast</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid </span><span style="color: Tomato;"><strong>redundant double-negation</strong></span><span style="color: Tomato;">.</span>

    <span class="token operator">!</span><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already</span>
    <span style="color: rgb(38, 148, 255);">be coerced to a boolean.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Boolean(</strong></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><strong>)</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token function">Boolean</span><span class="token punctuation">(</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
