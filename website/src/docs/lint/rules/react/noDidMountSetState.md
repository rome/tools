---
title: Lint Rule react/noDidMountSetState
layout: layouts/rule.liquid
description: prevent usage of `setState` in `componentDidMount`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-did-mount-set-state.md
eleventyNavigation:
	key: lint-rules/react/noDidMountSetState
	parent: lint-rules
	title: react/noDidMountSetState
---

# react/noDidMountSetState

<!-- GENERATED:START(hash:2fe34b6f26b8a68577a6a6d77f9cc270e2ca0b64,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent usage of `setState` in `componentDidMount`

**ESLint Equivalent:** [no-did-mount-set-state](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-did-mount-set-state.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:0a683b40f81e2bcd708fb015b06d44526bdbc1d0,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noDidMountSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid calling </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> in the </span><span style="color: Tomato;"><strong>componentDidMount</strong></span><span style="color: Tomato;"> method.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>      <span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Updating state after mounting causes a second render that can cause</span>
    <span style="color: DodgerBlue;">visual layout thrashing.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:2</span> <strong>lint/react/noDidMountSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid calling </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> in the </span><span style="color: Tomato;"><strong>componentDidMount</strong></span><span style="color: Tomato;"> method.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>      <span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
  <strong>  6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Updating state after mounting causes a second render that can cause</span>
    <span style="color: DodgerBlue;">visual layout thrashing.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:2</span> <strong>lint/react/noDidMountSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid calling </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> in the </span><span style="color: Tomato;"><strong>componentDidMount</strong></span><span style="color: Tomato;"> method.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>      <span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Updating state after mounting causes a second render that can cause</span>
    <span style="color: DodgerBlue;">visual layout thrashing.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
		<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:2</span> <strong>lint/react/noDidMountSetState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid calling </span><span style="color: Tomato;"><strong>this.setState</strong></span><span style="color: Tomato;"> in the </span><span style="color: Tomato;"><strong>componentDidMount</strong></span><span style="color: Tomato;"> method.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>      <span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
  <strong>  6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Updating state after mounting causes a second render that can cause</span>
    <span style="color: DodgerBlue;">visual layout thrashing.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">condition</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
				<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
			<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">condition</span> <span class="token operator">&amp;&amp;</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">condition</span> <span class="token punctuation">?</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">setState</span><span class="token punctuation">(</span><span class="token punctuation">{</span>
			<span class="token variable">name</span><span class="token punctuation">:</span> <span class="token string">&apos;John&apos;</span>
		<span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token variable">undefined</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
