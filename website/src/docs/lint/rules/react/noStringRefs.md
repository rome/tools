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

<!-- GENERATED:START(hash:dff53580430495074181c8a3ad2678a93edc4e3c,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:20</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>this.refs</strong></span><span style="color: Tomato;"> is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
     <strong> │ </strong>                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span>
    <span style="color: DodgerBlue;"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-ref</a></span>s
    <span style="color: DodgerBlue;"> for more information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span>
    <span style="color: DodgerBlue;"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-ref</a></span>s
    <span style="color: DodgerBlue;"> for more information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:18</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unclosed jsx expression container</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">We expected to find the closing character </span><span style="color: DodgerBlue;"><strong>}</strong></span><span style="color: DodgerBlue;"> here</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token operator">      </span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;hello&apos;</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span>
    <span style="color: DodgerBlue;"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-ref</a></span>s
    <span style="color: DodgerBlue;"> for more information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;$&quot;</span><span class="token punctuation">}</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token operator">&gt;</span><span class="token variable">Hello</span> <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:18</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unclosed jsx expression container</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">We expected to find the closing character </span><span style="color: DodgerBlue;"><strong>}</strong></span><span style="color: DodgerBlue;"> here</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token variable">hello$</span><span class="token punctuation">{</span><span class="token string">&quot;$&quot;</span><span class="token punctuation">}</span><span class="token punctuation">{</span><span class="token variable">index</span><span class="token punctuation">}</span><span class="token variable">$</span><span class="token punctuation">{</span><span class="token string">&quot;`&quot;</span><span class="token punctuation">}</span><span class="token operator">&gt;</span><span class="token variable">Hello</span> <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span>
    <span style="opacity: 0.8;">&rarr;</span><strong> │ </strong><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token string">&apos;hello&apos;</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:20</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>this.refs</strong></span><span style="color: Tomato;"> is a deprecated pattern.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">refs</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
     <strong> │ </strong>                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span>
    <span style="color: DodgerBlue;"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-ref</a></span>s
    <span style="color: DodgerBlue;"> for more information.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:7:14</span> <strong>lint/react/noStringRefs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using string literals in ref attributes is a deprecated pattern.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span>
    <span style="color: DodgerBlue;"><a href="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs">https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-ref</a></span>s
    <span style="color: DodgerBlue;"> for more information.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">const</span> <span class="token variable">component</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">hello</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>

	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">ref</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">c</span> <span class="token operator">=&gt;</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">hello</span> <span class="token operator">=</span> <span class="token variable">c</span><span class="token punctuation">}</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
