---
title: Lint Rule jsx-a11y/scope
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx-a11y/scope
	parent: lint-rules
	title: jsx-a11y/scope
---

# jsx-a11y/scope

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:9337c1bb7c57fa0b5b2be4c52e7dafbdb957f90c,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">scope</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/scope</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>scope</strong></span><span style="color: Tomato;"> attribute on elements other than </span><span style="color: Tomato;"><strong>th</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">scope</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using the scope attribute incorrectly on tables makes them difficult</span>
    <span style="color: DodgerBlue;">to navigate using the keyboard.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;div</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>scope={scope}</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;div</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token string">&apos;col&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/scope</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>scope</strong></span><span style="color: Tomato;"> attribute on elements other than </span><span style="color: Tomato;"><strong>th</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token string">&apos;col&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Using the scope attribute incorrectly on tables makes them difficult</span>
    <span style="color: DodgerBlue;">to navigate using the keyboard.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;div</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>scope=&quot;col&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;div</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">th</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">scope</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">th</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">th</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token string">&apos;col&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">th</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
