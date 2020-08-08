---
title: Lint Rule react/noThisInSFC
layout: layouts/rule.liquid
description: report `this` being used in stateless components
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-this-in-sfc.md
eleventyNavigation:
	key: lint-rules/react/noThisInSFC
	parent: lint-rules
	title: react/noThisInSFC
---

# react/noThisInSFC

<!-- GENERATED:START(hash:d168462932920b62d310a2b13a1518d6dd055fc6,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
report `this` being used in stateless components

**ESLint Equivalent:** [no-this-in-sfc](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-this-in-sfc.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2e611eeaea8ce642b29080b31320453871a32796,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:44</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unterminated regular expression</span>

    <span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token operator">&lt;</span><span class="token regex">/div&gt;</span>
                                                <span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:8</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
     <strong> │ </strong>          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:17</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
     <strong> │ </strong>                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong>  4</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:8</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
     <strong> │ </strong>          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:17</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
     <strong> │ </strong>                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong>  4</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">context</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">context</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:4</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong>  3</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">context</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:23</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">context</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">context</span><span class="token punctuation">;</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:17</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">context</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">context</span><span class="token punctuation">;</span>
     <strong> │ </strong>                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
  <strong>  4</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token punctuation">(</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:17</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">loading</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">Loader</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:5</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">loading</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">Loader</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:7:4</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">loading</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">;</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">loading</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> &lt;<span class="token attr-name">Loader</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:21</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">loading</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">;</span>
     <strong> │ </strong>                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">;</span>
  <strong>  4</strong><strong> │ </strong>  <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">loading</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:17</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">memo</span><span class="token punctuation">(</span>
	<span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> <span class="token punctuation">(</span>
			&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
		<span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:9</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">return</span> <span class="token punctuation">(</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>      &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
     <strong> │ </strong>            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    <span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token attr-name">div</span>&gt;
		<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
	&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:3</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
  <strong>  2</strong><strong> │ </strong>  &lt;<span class="token attr-name">div</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
     <strong> │ </strong>     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token attr-name">div</span>&gt;
		<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
	&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:3</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
  <strong>  2</strong><strong> │ </strong>  &lt;<span class="token attr-name">div</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
     <strong> │ </strong>     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">memo</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span>
<span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noThisInSFC</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>this</strong></span><span style="color: Tomato;"> in stateless functional components.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">memo</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
  <strong>  2</strong><strong> │ </strong>    &lt;<span class="token attr-name">div</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>      <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    &lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">)</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The </span><span style="color: DodgerBlue;"><strong>this</strong></span><span style="color: DodgerBlue;"> keyword has no binding in functional components. Use hooks</span>
    <span style="color: DodgerBlue;">instead.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">bar</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">context</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> <span class="token punctuation">(</span>
			&lt;<span class="token attr-name">div</span>&gt;
				<span class="token punctuation">{</span><span class="token variable">context</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
			&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
		<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">context</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">context</span><span class="token punctuation">;</span>
	<span class="token keyword">const</span> <span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token variable">props</span><span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token punctuation">{</span> <span class="token variable">bar</span> <span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token punctuation">{</span> <span class="token variable">foo</span> <span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">MyComponent</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;some jsx&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">}</span>

	<span class="token variable">render</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> <span class="token string">&apos;content&apos;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">memo</span><span class="token punctuation">(</span>
	<span class="token keyword">function</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">return</span> <span class="token punctuation">(</span>
			&lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
		<span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token attr-name">div</span>&gt;
		<span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
	&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">forwardRef</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
	&lt;<span class="token attr-name">div</span>&gt;
		<span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
	&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
<span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">Foo</span> <span class="token operator">=</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">memo</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">,</span> <span class="token variable">ref</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">(</span>
		&lt;<span class="token attr-name">div</span>&gt;
			<span class="token punctuation">{</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">bar</span><span class="token punctuation">}</span>
		&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
	<span class="token punctuation">)</span>
<span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
