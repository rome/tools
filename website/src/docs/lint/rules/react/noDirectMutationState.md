---
title: Lint Rule react/noDirectMutationState
layout: layouts/rule.liquid
description: prevent direct mutation of `this.state`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-direct-mutation-state.md
eleventyNavigation:
	key: lint-rules/react/noDirectMutationState
	parent: lint-rules
	title: react/noDirectMutationState
---

# react/noDirectMutationState

<!-- GENERATED:START(hash:1976d33619f47ca551904b3d832e0c002d9d88bf,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent direct mutation of `this.state`

**ESLint Equivalent:** [no-direct-mutation-state](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-direct-mutation-state.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:babf843adcefbd9110e8524ec6ee90556268e94d,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token operator">++</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token operator">++</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token operator">--</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token operator">--</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">+=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">+=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">delete</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">delete</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">last</span> <span class="token operator">=</span> <span class="token string">&apos;baz&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">last</span> <span class="token operator">=</span> <span class="token string">&apos;baz&apos;</span><span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token variable">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span>
  <span class="token punctuation">}</span>
  <span class="token variable">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:6:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token variable">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span>
    <span class="token variable">doSomethingAsync</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
      <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token string">&apos;bad&apos;</span><span class="token punctuation">;</span>
    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:5:6</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span>
  <strong>  4</strong><strong> │ </strong>    <span class="token variable">doSomethingAsync</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token string">&apos;bad&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token variable">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Calling </span><span style="color: DodgerBlue;"><strong>setState()</strong></span><span style="color: DodgerBlue;"> after mutating </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly may replace the</span>
    <span style="color: DodgerBlue;">mutation you made. The only place you may set </span><span style="color: DodgerBlue;"><strong>this.state</strong></span><span style="color: DodgerBlue;"> directly is</span>
    <span style="color: DodgerBlue;">in a constructor of a react class component.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">var</span> <span class="token variable">obj</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">state</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
    <span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">name</span> <span class="token operator">=</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">var</span> <span class="token variable">Hello</span> <span class="token operator">=</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">;</span>
<span class="token variable">module</span><span class="token punctuation">.</span><span class="token variable">exports</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token punctuation">{</span>
  <span class="token variable">getFoo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">OneComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">class</span> <span class="token variable">AnotherComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
      <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
        <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
      <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">typeof</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token attr-name">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">SuperExample</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token variable">someOtherFunction</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
