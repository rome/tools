---
title: Lint Rule jsx-a11y/noAriaUnsupportedElements
layout: layouts/rule.liquid
description: enforce that elements that do not support ARIA roles, states, and properties do not have those attributes
eslint-rule: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/aria-unsupported-elements.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noAriaUnsupportedElements
	parent: lint-rules
	title: jsx-a11y/noAriaUnsupportedElements
---

# jsx-a11y/noAriaUnsupportedElements

<!-- GENERATED:START(hash:55d6d38ab5c8f7b4fb7076d1c8db356e484db55a,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce that elements that do not support ARIA roles, states, and properties do not have those attributes

**ESLint Equivalent:** [aria-unsupported-elements](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/aria-unsupported-elements.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:3a0952b604a85b060b70dc943ccf20e176555339,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">meta</span> <span class="token attr-name">charset</span><span class="token operator">=</span><span class="token string">&apos;UTF-8&apos;</span> <span class="token attr-name">aria-hidden</span><span class="token operator">=</span><span class="token string">&apos;false&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">meta</span> <span class="token attr-name">charset</span><span class="token operator">=</span><span class="token string">&apos;UTF-8&apos;</span> <span class="token attr-name">aria-hidden</span><span class="token operator">=</span><span class="token string">&apos;false&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">meta</span> <span class="token attr-name">charset</span><span class="token operator">=</span><span class="token string">&apos;UTF-8&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;meta&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">meta</span> <span class="token attr-name">charset</span><span class="token operator">=</span><span class="token string">&apos;UTF-8&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;meta&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">html</span> <span class="token attr-name">aria-required</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">html</span> <span class="token attr-name">aria-required</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">html</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;html&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">html</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;html&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">html</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">script</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token string">&apos;script&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">script</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">script</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token string">&apos;script&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">script</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">script</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;script&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">script</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">script</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;script&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">script</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">style</span> <span class="token attr-name">aria-labelledby</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">style</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">style</span> <span class="token attr-name">aria-labelledby</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">style</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">style</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;style&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">style</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noAriaUnsupportedElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>role</strong></span><span style="color: Tomato;"> attribute and </span><span style="color: Tomato;"><strong>aria-*</strong></span><span style="color: Tomato;"> attributes when using </span><span style="color: Tomato;"><strong>meta</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;">,</span>
    <span style="color: Tomato;"><strong>script</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">style</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;style&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">style</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using roles on elements that do not support them can cause issues</span>
    <span style="color: DodgerBlue;">with screen readers.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">meta</span> <span class="token attr-name">charset</span><span class="token operator">=</span><span class="token string">&apos;UTF-8&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">html</span> <span class="token attr-name">maria</span><span class="token operator">=</span><span class="token string">&apos;text&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">script</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">script</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">style</span> <span class="token attr-name">parole</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">style</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
