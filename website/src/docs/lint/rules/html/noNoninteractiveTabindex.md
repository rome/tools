---
title: Lint Rule a11y/noNoninteractiveTabindex
layout: layouts/rule.liquid
description: "`tabIndex` should only be declared on interactive elements"
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-noninteractive-tabindex.md
eleventyNavigation:
	key: lint-rules/a11y/noNoninteractiveTabindex
	parent: lint-rules
	title: a11y/noNoninteractiveTabindex
---

# a11y/noNoninteractiveTabindex

<!-- GENERATED:START(hash:28e7716442ffc9de2a99e6c150eb01cd17683ae1,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
"`tabIndex` should only be declared on interactive elements"

**ESLint Equivalent:** [no-noninteractive-tabindex](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-noninteractive-tabindex.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:64d8974b64d8fcb24439ff7c841e3557d1a30bc2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx-a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx-a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:9</span> <strong>lint/jsx-a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">button</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">button</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;button&apos;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;0&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;-1&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">MyButton</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;-1&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&apos;-1&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">button</span> <span class="token attr-name">tabindex</span><span class="token operator">=</span><span class="token string">&apos;-1&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">button</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
