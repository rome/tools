---
title: Lint Rule jsx-a11y/anchorIsValid
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx-a11y/anchorIsValid
	parent: lint-rules
	title: jsx-a11y/anchorIsValid
---

# jsx-a11y/anchorIsValid

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:14c8a8bb2946441800f0e40055ff5c4436d8e521,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">null</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">null</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;#&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">#</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;javascript:void(0)&apos;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;javascript:void(0)&apos;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> attribute for the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">javascript:void(0)</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/anchorIsValid</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Anchor elements should only be used for default section or page</span>
    <span style="color: DodgerBlue;">navigation.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;https://github.com&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#section&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;/foo/bar&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">someValidPath</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;https://github.com&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;#section&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;/foo/bar&apos;</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">someValidPath</span><span class="token punctuation">}</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
