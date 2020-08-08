---
title: Lint Rule react/useRenderReturn
layout: layouts/rule.liquid
description: This rule makes sure the render function is returning content
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/require-render-return.md
eleventyNavigation:
	key: lint-rules/react/useRenderReturn
	parent: lint-rules
	title: react/useRenderReturn
---

# react/useRenderReturn

This rule makes sure the render function is returning content.

<!-- GENERATED:START(hash:47d2582223ebed44beddc17da0d83a41adffc7c9,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [require-render-return](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/require-render-return.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:4225a3d0bf21186fe860641018c62aec1cc88d07,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:10</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span> <span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">.</span><span class="token variable">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">num</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt; Foo &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:10</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span> <span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">.</span><span class="token variable">map</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">num</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt; Foo &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">Component</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">Component</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">Component</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">Component</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">PureComponent</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:16</span> <strong>lint/react/useRenderReturn</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> method on a component must return content.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">import</span> <span class="token variable">React</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">PureComponent</span><span class="token punctuation">}</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
     <strong> │ </strong>                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Foo&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span> <span class="token variable">React</span> <span class="token keyword">from</span> <span class="token string">&apos;react&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Foo&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
		<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> <span class="token keyword">void</span> <span class="token number">0</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> &lt;&gt;&lt;<span class="token operator">/</span>&gt; <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>&lt;&gt;&lt;<span class="token operator">/</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token keyword">extends</span> <span class="token variable">Bar</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
