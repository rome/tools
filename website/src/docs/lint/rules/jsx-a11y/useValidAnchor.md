---
title: Lint Rule jsx-a11y/useValidAnchor
layout: layouts/rule.liquid
description: enforce all anchors are valid, navigable elements
eslint-rule: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/anchor-is-valid.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/useValidAnchor
	parent: lint-rules
	title: jsx-a11y/useValidAnchor
---

# jsx-a11y/useValidAnchor

<!-- GENERATED:START(hash:d686bd489222ed8f4bb5fb2c49c6741730776927,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce all anchors are valid, navigable elements

**ESLint Equivalent:** [anchor-is-valid](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/anchor-is-valid.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:41a51a5aa0350ee4a430e3050caf36bcfa9d48bd,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">null</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useValidAnchor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default section or page</span>
    <span style="color: rgb(38, 148, 255);">navigation.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;https://github.com&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#section&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;/foo/bar&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">someValidPath</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;https://github.com&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#section&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;/foo/bar&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">someValidPath</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
