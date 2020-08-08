---
title: Lint Rule react/useSortComp
layout: layouts/rule.liquid
description: enforce component methods order
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/sort-comp.md
eleventyNavigation:
	key: lint-rules/react/useSortComp
	parent: lint-rules
	title: react/useSortComp
---

# react/useSortComp

This rule enforces methods and properties order. When creating React components it is more convenient to always follow the same organisation for method order to help you easily find lifecycle methods, event handlers, etc.

<!-- GENERATED:START(hash:cb48f03eeaac6c20f8d21fe7bebd86070c0c51db,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [sort-comp](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/sort-comp.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:5b22ee3464a6e3f718f145f32b73d48ad6e1db38,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must force a lifecycle method to be placed before render</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token variable">displayName</span> <span class="token operator">=</span> <span class="token string">&apos;Hello&apos;</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:1</span> <strong>lint/react/useSortComp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> should be placed after </span><span style="color: Tomato;"><strong>displayName</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// Must force a lifecycle method to be placed before render</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">When creating React components it is more convenient to always follow</span>
    <span style="color: DodgerBlue;">the same organisation for method order to help you easily fin</span>d
    <span style="color: DodgerBlue;">lifecycle methods, event handlers, etc.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must force a custom method to be placed before render</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token variable">onClick</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:1</span> <strong>lint/react/useSortComp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> should be placed after </span><span style="color: Tomato;"><strong>onClick</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// Must force a custom method to be placed before render</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">When creating React components it is more convenient to always follow</span>
    <span style="color: DodgerBlue;">the same organisation for method order to help you easily fin</span>d
    <span style="color: DodgerBlue;">lifecycle methods, event handlers, etc.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must force a custom method to be placed before render, even in function</span>
<span class="token keyword">var</span> <span class="token variable">Hello</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
		<span class="token variable">render</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
		<span class="token variable">onClick</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:2</span> <strong>lint/react/useSortComp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;"> should be placed after </span><span style="color: Tomato;"><strong>onClick</strong></span><span style="color: Tomato;">.</span>

  <strong>  2</strong><strong> │ </strong><span class="token keyword">var</span> <span class="token variable">Hello</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token variable">render</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>      <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>    <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">When creating React components it is more convenient to always follow</span>
    <span style="color: DodgerBlue;">the same organisation for method order to help you easily fin</span>d
    <span style="color: DodgerBlue;">lifecycle methods, event handlers, etc.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Type Annotations should not be at the top by default</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">props</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">text</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">state</span><span class="token punctuation">:</span> <span class="token variable">Object</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">text</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:1</span> <strong>lint/react/useSortComp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>props</strong></span><span style="color: Tomato;"> should be placed after </span><span style="color: Tomato;"><strong>render</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// Type Annotations should not be at the top by default</span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token variable">props</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">text</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token variable">state</span><span class="token punctuation">:</span> <span class="token variable">Object</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">When creating React components it is more convenient to always follow</span>
    <span style="color: DodgerBlue;">the same organisation for method order to help you easily fin</span>d
    <span style="color: DodgerBlue;">lifecycle methods, event handlers, etc.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must validate a full class</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">displayName</span> <span class="token operator">=</span> <span class="token string">&apos;&apos;</span>
  <span class="token variable">propTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">contextTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">childContextTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">mixins</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>
  <span class="token variable">statics</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getDefaultProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getInitialState</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getChildContext</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must validate a class with missing groups</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must put a custom method in &apos;everything-else&apos;</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">onClick</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">button</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">onClick</span><span class="token punctuation">}</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">button</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must validate a full React class</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">displayName</span> <span class="token operator">=</span> <span class="token string">&apos;&apos;</span>
  <span class="token variable">propTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">contextTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">childContextTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">mixins</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>
  <span class="token variable">statics</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getDefaultProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getInitialState</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getChildContext</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">UNSAFE_componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">UNSAFE_componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">UNSAFE_componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">getSnapshotBeforeUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentDidCatch</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must validate React 16.3 lifecycle methods with the default parser</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">getSnapshotBeforeUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidCatch</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">testInstanceMethod</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token punctuation">(</span>&lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must validate a full React 16.3 ES6 class</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">static</span> <span class="token variable">displayName</span> <span class="token operator">=</span> <span class="token string">&apos;&apos;</span>
	<span class="token variable">static</span> <span class="token variable">propTypes</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">static</span> <span class="token variable">defaultProps</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillMount</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidMount</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillReceiveProps</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">shouldComponentUpdate</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">UNSAFE_componentWillUpdate</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">getSnapshotBeforeUpdate</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidUpdate</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentDidCatch</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">componentWillUnmount</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">testArrowMethod</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">testInstanceMethod</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">render</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>&lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">)</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must allow us to use &apos;constructor&apos; as a method name</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">displayName</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must ignore stateless components</span>
<span class="token keyword">function</span> <span class="token variable">Hello</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Must ignore stateless components (arrow function with explicit return)</span>
<span class="token keyword">var</span> <span class="token variable">Hello</span> <span class="token operator">=</span> <span class="token variable">props</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Non-react classes should be ignored, even in expressions</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">text</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token variable">props</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">text</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">state</span><span class="token punctuation">:</span> <span class="token variable">Object</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// Non-react classes should be ignored, even in expressions</span>
<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token keyword">class</span> <span class="token punctuation">{</span>
	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">text</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token variable">props</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">text</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">state</span><span class="token punctuation">:</span> <span class="token variable">Object</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// static lifecycle methods can be grouped (with lifecycle)</span>
<span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
	<span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">static</span> <span class="token variable">propTypes</span><span class="token punctuation">;</span>
  <span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token variable">foo</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">static</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">static</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;some-str&apos;</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token variable">static</span> <span class="token variable">bar</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">static</span> <span class="token variable">getDerivedStateFromProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
  <span class="token variable">static</span> <span class="token variable">bar</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
  <span class="token variable">static</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <span class="token variable">bar</span><span class="token punctuation">;</span>
  <span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">return</span> <span class="token boolean">null</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
