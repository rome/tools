---
title: Lint Rule react/useStylePropObject
layout: layouts/rule.liquid
description: enforce style prop value is an object
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/style-prop-object.md
eleventyNavigation:
	key: lint-rules/react/useStylePropObject
	parent: lint-rules
	title: react/useStylePropObject
---

# react/useStylePropObject

This rule enforces the value of `style` to be a object. React will ignore non-object style props, even valid JSON strings.

<!-- GENERATED:START(hash:e89cfe0c7e887b8306dc9ca7c32a4b9604f6765e,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [style-prop-object](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/style-prop-object.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:514cfe5849941754e1f184d53fc3ac90545765bc,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">true</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/useStylePropObject</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop value must be an object.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">true</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React will ignore non-object style props, even valid JSON strings.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">1</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/useStylePropObject</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop value must be an object.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">1</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React will ignore non-object style props, even valid JSON strings.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/useStylePropObject</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop value must be an object.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React will ignore non-object style props, even valid JSON strings.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&quot;color: &apos;red&apos;&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/useStylePropObject</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop value must be an object.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&quot;color: &apos;red&apos;&quot;</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React will ignore non-object style props, even valid JSON strings.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;color: &apos;red&apos;&quot;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/useStylePropObject</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop value must be an object.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;color: &apos;red&apos;&quot;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React will ignore non-object style props, even valid JSON strings.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">{</span><span class="token variable">color</span><span class="token punctuation">:</span> <span class="token string">&apos;red&apos;</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
