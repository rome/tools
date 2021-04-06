---
title: Lint Rule jsx-a11y/useHeadingContent
layout: layouts/rule.liquid
description: enforce heading (`h1`, `h2`, etc) elements contain accessible content
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/heading-has-content.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/useHeadingContent
	parent: lint-rules
	title: jsx-a11y/useHeadingContent
---

# jsx-a11y/useHeadingContent

<!-- GENERATED:START(hash:b9b6d25c770d35c11da9d0a74c305c6dd60d7ecf,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce heading (`h1`, `h2`, etc) elements contain accessible content

**ESLint Equivalent:** [heading-has-content](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/heading-has-content.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:9be87697d3d80805258f66a945bb27ba50baab90,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useHeadingContent</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">h1</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">All headings on a page should have content that is accessible to</span>
    <span style="color: rgb(38, 148, 255);">screen readers.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">TextWrapper</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useHeadingContent</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">TextWrapper</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">All headings on a page should have content that is accessible to</span>
    <span style="color: rgb(38, 148, 255);">screen readers.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">div</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useHeadingContent</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">div</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">All headings on a page should have content that is accessible to</span>
    <span style="color: rgb(38, 148, 255);">screen readers.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span>&gt;heading&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">TextWrapper</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span> <span class="token attr-name">dangerouslySetInnerHTML</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;heading&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">h1</span>&gt;&lt;<span class="token variable">div</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;visible content&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
