---
title: Lint Rule jsx-a11y/useAnchorContent
layout: layouts/rule.liquid
description: enforce that anchors have content and that the content is accessible to screen readers
eslint-rule: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/anchor-has-content.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/useAnchorContent
	parent: lint-rules
	title: jsx-a11y/useAnchorContent
---

# jsx-a11y/useAnchorContent

<!-- GENERATED:START(hash:73f844f67ec1128f1889ee6f4a37504f4fdf71b1,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce that anchors have content and that the content is accessible to screen readers

**ESLint Equivalent:** [anchor-has-content](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/anchor-has-content.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:ecbd13c85746ffee2894f0f731ae60f81399ae7e,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useAnchorContent</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>anchor</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">a</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">All links on a page should have content that is accessible to screen</span>
    <span style="color: DodgerBlue;">readers.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span>&gt;&lt;<span class="token attr-name">TextWrapper</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/jsx-a11y/useAnchorContent</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>anchor</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">a</span>&gt;&lt;<span class="token attr-name">TextWrapper</span> <span class="token attr-name">aria-hidden</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">All links on a page should have content that is accessible to screen</span>
    <span style="color: DodgerBlue;">readers.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span>&gt;Anchor Content!&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span>&gt;&lt;<span class="token attr-name">TextWrapper</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">dangerouslySetInnerHTML</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;foo&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span>&gt;&lt;<span class="token attr-name">TextWrapper</span> <span class="token attr-name">aria-hidden</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">true</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt; visible content&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
