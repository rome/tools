---
title: Lint Rule react/noArrayIndexKey
layout: layouts/rule.liquid
description: prevent usage of Array index in keys
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-array-index-key.md
eleventyNavigation:
	key: lint-rules/react/noArrayIndexKey
	parent: lint-rules
	title: react/noArrayIndexKey
---

# react/noArrayIndexKey

<!-- GENERATED:START(hash:4ecac919a362c76e6034d304e18d1486db600512,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent usage of Array index in keys

**ESLint Equivalent:** [no-array-index-key](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-array-index-key.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:0db20c265c239d6e318d47b34c94bb97f998d2a9,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:7</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:8</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:14</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
	<span class="token keyword">return</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:13</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:19</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:8</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
	<span class="token keyword">return</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:13</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:7</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:7</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:8</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:14</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
	<span class="token keyword">return</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:13</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:19</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:8</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
	<span class="token keyword">return</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:13</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:7</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Children</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">children</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">child</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:9</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:7</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span><span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">index</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:8</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">filter</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">some</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">every</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">find</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">findIndex</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:18</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduce</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:19</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduce</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:26</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduceRight</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:19</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduceRight</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token variable">index</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:26</span> <strong>lint/react/noArrayIndexKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using array index as key property in an element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">https://reactjs.org/docs/lists-and-keys.html#keys</a></span><span style="color: rgb(38, 148, 255);"> for more</span>
    <span style="color: rgb(38, 148, 255);">information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> &lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">map</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">cloneElement</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">key</span><span class="token punctuation">:</span> <span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span> <span class="token punctuation">}</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">forEach</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">filter</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">filter</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">some</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">some</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">every</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">every</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">find</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">find</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">findIndex</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">findIndex</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">otherThings</span><span class="token punctuation">.</span><span class="token function">push</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduce</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduce</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduceRight</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	<span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">things</span><span class="token punctuation">.</span><span class="token function">reduceRight</span><span class="token punctuation">(</span><span class="token keyword">function</span><span class="token punctuation">(</span><span class="token variable">collection</span><span class="token punctuation">,</span> <span class="token variable">thing</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">collection</span><span class="token punctuation">.</span><span class="token function">concat</span><span class="token punctuation">(</span>&lt;<span class="token variable">Hello</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">thing</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
