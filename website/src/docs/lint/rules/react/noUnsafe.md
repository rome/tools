---
title: Lint Rule react/noUnsafe
layout: layouts/rule.liquid
description: prevent usage of unsafe lifecycle methods
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-unsafe.md
eleventyNavigation:
	key: lint-rules/react/noUnsafe
	parent: lint-rules
	title: react/noUnsafe
---

# react/noUnsafe

<!-- GENERATED:START(hash:a8517e3f3ecb9ca7249faf73c5910758b00bbb64,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent usage of unsafe lifecycle methods

**ESLint Equivalent:** [no-unsafe](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-unsafe.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:3ac55f6d5d9422aeac138ae34b357b8c0944fcef,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">UNSAFE_componentWillMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noUnsafe</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>UNSAFE_componentWillMount</strong></span><span style="color: Tomato;"> method is unsafe for use in async</span>
    <span style="color: Tomato;">rendering. Use the </span><span style="color: Tomato;"><strong>componentDidMount</strong></span><span style="color: Tomato;"> method instead.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html">https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html</a></span>
    <span style="color: rgb(38, 148, 255);">for more information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">UNSAFE_componentWillReceiveProps</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noUnsafe</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>UNSAFE_componentWillReceiveProps</strong></span><span style="color: Tomato;"> method is unsafe for use in</span>
    <span style="color: Tomato;">async rendering. Use the </span><span style="color: Tomato;"><strong>getDerivedStateFromProps</strong></span><span style="color: Tomato;"> method instead.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html">https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html</a></span>
    <span style="color: rgb(38, 148, 255);">for more information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">UNSAFE_componentWillUpdate</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:1</span> <strong>lint/react/noUnsafe</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>UNSAFE_componentWillUpdate</strong></span><span style="color: Tomato;"> method is unsafe for use in async</span>
    <span style="color: Tomato;">rendering. Use the </span><span style="color: Tomato;"><strong>componentDidUpdate</strong></span><span style="color: Tomato;"> method instead.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">See </span>
    <span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html">https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html</a></span>
    <span style="color: rgb(38, 148, 255);">for more information.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Hello</span> <span class="token keyword">extends</span> <span class="token variable">React</span><span class="token punctuation">.</span><span class="token variable">Component</span> <span class="token punctuation">{</span>
	<span class="token function">componentDidMount</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
