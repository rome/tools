---
title: Lint Rule a11y/noNoninteractiveTabindex
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/a11y/noNoninteractiveTabindex
	parent: lint-rules
	title: a11y/noNoninteractiveTabindex
---

# a11y/noNoninteractiveTabindex

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2aa9b7aa240104ddf52775427fd1d3568153570b,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;article&quot;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;article&quot;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:9</span> <strong>lint/a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    &lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:5</span> <strong>lint/a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;article&quot;</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:5</span> <strong>lint/a11y/noNoninteractiveTabindex</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> on an element that is not interactive.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;article&quot;</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Adding non-interactive elements to the keyboard navigation flow can</span>
    <span style="color: rgb(38, 148, 255);">confuse users.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">button</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">button</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;button&quot;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;article&quot;</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;-1&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">MyButton</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;-1&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;-1&quot;</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">article</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">button</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;-1&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">button</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">button</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">button</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;button&quot;</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;article&quot;</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;-1&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">article</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;-1&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">article</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;-1&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">button</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;-1&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">button</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
