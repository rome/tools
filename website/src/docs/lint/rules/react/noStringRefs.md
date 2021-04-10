---
title: Lint Rule react/noStringRefs
layout: layouts/rule.liquid
description: prevent string definitions for references and prevent referencing `this.refs`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-string-refs.md
eleventyNavigation:
	key: lint-rules/react/noStringRefs
	parent: lint-rules
	title: react/noStringRefs
---

# react/noStringRefs

<!-- GENERATED:START(hash:d13424d62bfae0fe5e1cd6e473db8f3d59064abc,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent string definitions for references and prevent referencing `this.refs`

**ESLint Equivalent:** [no-string-refs](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-string-refs.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:6731425236948136d1a9b2bcd8c16e77687b3f62,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:20</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>this.refs</strong></span><span style="color: Tomato;"> is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
     <strong> │ </strong>                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs</a></span>
    <span style="color: rgb(38, 148, 255);"> for more information.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs</a></span>
    <span style="color: rgb(38, 148, 255);"> for more information.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:18</span> <strong>parse(js)</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unclosed </span><span style="color: Tomato;"><strong>jsx expression container</strong></span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token operator">      </span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">We expected to find the closing character </span><span style="color: rgb(38, 148, 255);"><strong>}</strong></span><span style="color: rgb(38, 148, 255);"> here</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token operator">      </span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs</a></span>
    <span style="color: rgb(38, 148, 255);"> for more information.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;$&quot;</span><span class="token punctuation">}</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token operator">&gt;</span><span class="token variable">Hello</span> <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:18</span> <strong>parse(js)</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unclosed </span><span style="color: Tomato;"><strong>jsx expression container</strong></span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;$&quot;</span><span class="token punctuation">}</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token operator">&gt;</span><span class="token variable">Hello</span> <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">We expected to find the closing character </span><span style="color: rgb(38, 148, 255);"><strong>}</strong></span><span style="color: rgb(38, 148, 255);"> here</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;$&quot;</span><span class="token punctuation">}</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token operator">&gt;</span><span class="token variable">Hello</span> <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:20</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>this.refs</strong></span><span style="color: Tomato;"> is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
     <strong> │ </strong>                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs</a></span>
    <span style="color: rgb(38, 148, 255);"> for more information.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:7:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong>  6</strong><strong> │ </strong>  <span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  9</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs</a></span>
    <span style="color: rgb(38, 148, 255);"> for more information.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token function">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token variable">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">c</span> <span class="token operator">=&gt;</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">hello</span> <span class="token operator">=</span> <span class="token variable">c</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
