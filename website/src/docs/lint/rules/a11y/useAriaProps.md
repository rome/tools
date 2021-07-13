---
title: Lint Rule a11y/useAriaProps
layout: layouts/rule.liquid
description: enforce all `aria-*` props are valid
eslint-rule: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/aria-props.md
eleventyNavigation:
	key: lint-rules/a11y/useAriaProps
	parent: lint-rules
	title: a11y/useAriaProps
---

# a11y/useAriaProps

<!-- GENERATED:START(hash:c0d23fea100203fb8d3e212c4e8559688dba9f84,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce all `aria-*` props are valid

**ESLint Equivalent:** [aria-props](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/aria-props.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2d5c78bb96314faf52815c7668a660d9e7b696ec,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;&quot;</span> <span class="token attr-name">aria-labell</span><span class="token operator">=</span><span class="token string">&quot;&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1:20</span> <strong>lint/a11y/useAriaProps</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-labell</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    &lt;<span class="token variable">input</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token string">&quot;&quot;</span> <span class="token attr-name">aria-labell</span><span class="token operator">=</span><span class="token string">&quot;&quot;</span> <span class="token operator">/</span>&gt;
                        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> ARIA Spelling Mistake

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aria-label</span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;">=&quot;&quot;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aria-label=&quot;&quot;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">aria-</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1:5</span> <strong>lint/a11y/useAriaProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">aria-</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">aria-labeledby</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1:5</span> <strong>lint/a11y/useAriaProps</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-labeledby</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">aria-labeledby</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> ARIA Spelling Mistake

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aria-labeledby=&quot;foobar&quot;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aria-label</span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;">edby=&quot;foobar&quot;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">aria-skldjfaria-klajsd</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1:5</span> <strong>lint/a11y/useAriaProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-skldjfaria-klajsd</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    &lt;<span class="token variable">div</span> <span class="token attr-name">aria-skldjfaria-klajsd</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">class</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;&quot;</span> <span class="token attr-name">aria-labell</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:16</span> <strong>lint/a11y/useAriaProps</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-labell</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">class</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;&quot;</span> <span class="token attr-name">aria-labell</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;&quot;</span> <span class="token punctuation">/&gt;</span>
                    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> ARIA Spelling Mistake

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aria-label</span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;">=&quot;&quot;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aria-label=&quot;&quot;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:5</span> <strong>lint/a11y/useAriaProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-labeledby</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:5</span> <strong>lint/a11y/useAriaProps</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-labeledby</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-labeledby</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> ARIA Spelling Mistake

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aria-labeledby=&quot;foobar&quot;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aria-label</span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;">edby=&quot;foobar&quot;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-skldjfaria-klajsd</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:5</span> <strong>lint/a11y/useAriaProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>aria-skldjfaria-klajsd</strong></span><span style="color: Tomato;"> is an invalid ARIA attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria-skldjfaria-klajsd</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">aria</span><span class="token operator">=</span><span class="token string">&quot;wee&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">abcARIAdef</span><span class="token operator">=</span><span class="token string">&quot;true&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">fooaria-foobar</span><span class="token operator">=</span><span class="token string">&quot;true&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">div</span> <span class="token attr-name">fooaria-hidden</span><span class="token operator">=</span><span class="token string">&quot;true&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;text&quot;</span> <span class="token attr-name">aria-errormessage</span><span class="token operator">=</span><span class="token string">&quot;foobar&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">aria</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;wee&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">abcARIAdef</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;true&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">fooaria-foobar</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;true&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span> <span class="token attr-name">fooaria-hidden</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;true&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;text&quot;</span> <span class="token attr-name">aria-errormessage</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foobar&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
