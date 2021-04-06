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

<!-- GENERATED:START(hash:e4c678dbdfb1175b820706e726f269c0459cccfe,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token operator">++</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token operator">++</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token operator">--</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token operator">--</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">+=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">+=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">delete</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">delete</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">last</span> <span class="token operator">=</span> <span class="token string">&apos;baz&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">last</span> <span class="token operator">=</span> <span class="token string">&apos;baz&apos;</span><span class="token punctuation">;</span>
  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:4:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  2</strong><strong> │ </strong>  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">first</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">.</span><span class="token variable">last</span> <span class="token operator">=</span> <span class="token string">&apos;baz&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <strong>  6</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token function">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span>
  <span class="token punctuation">}</span>
  <span class="token function">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:6:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong>  <span class="token function">someFn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  8</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span>
    <span class="token function">doSomethingAsync</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
      <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token string">&apos;bad&apos;</span><span class="token punctuation">;</span>
    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:5:6</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  3</strong><strong> │ </strong>    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span>
  <strong>  4</strong><strong> │ </strong>    <span class="token function">doSomethingAsync</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>      <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token string">&apos;bad&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>    <span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong>  7</strong><strong> │ </strong>  <span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">shouldComponentUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentDidUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:3:4</span> <strong>lint/react/noDirectMutationState</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid mutating </span><span style="color: Tomato;"><strong>this.state</strong></span><span style="color: Tomato;"> directly.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">componentWillUnmount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Calling </span><span style="color: rgb(38, 148, 255);"><strong>setState()</strong></span><span style="color: rgb(38, 148, 255);"> after mutating </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly may replace the</span>
    <span style="color: rgb(38, 148, 255);">mutation you made. The only place you may set </span><span style="color: rgb(38, 148, 255);"><strong>this.state</strong></span><span style="color: rgb(38, 148, 255);"> directly is</span>
    <span style="color: rgb(38, 148, 255);">in a constructor of a react class component.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">var</span> <span class="token variable">obj</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">state</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
    <span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">name</span> <span class="token operator">=</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">var</span> <span class="token variable">Hello</span> <span class="token operator">=</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">;</span>
<span class="token variable">module</span><span class="token punctuation">.</span><span class="token variable">exports</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token punctuation">{</span>
  <span class="token function">getFoo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">PureComponent</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;bar&apos;</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">OneComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">class</span> <span class="token variable">AnotherComponent</span> <span class="token keyword">extends</span> <span class="token variable">Component</span> <span class="token punctuation">{</span>
      <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
        <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
      <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">testFunc</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">typeof</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span><span class="token punctuation">.</span><span class="token variable">person</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">;</span>
    <span class="token keyword">return</span> &lt;<span class="token variable">div</span>&gt;Hello <span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">props</span><span class="token punctuation">.</span><span class="token variable">name</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">SuperExample</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">constructor</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">super</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">class</span> <span class="token variable">Example</span> <span class="token keyword">extends</span> <span class="token variable">SuperExample</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
  <span class="token function">someOtherFunction</span><span class="token punctuation">(</span><span class="token variable">props</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">state</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
      <span class="token variable">count</span><span class="token punctuation">:</span> <span class="token number">0</span>
    <span class="token punctuation">}</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
