---
title: Lint Rule a11y/useAltText
layout: layouts/rule.liquid
showHero: false
description: Checks that images have a valid alternative text
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/alt-text.md
eleventyNavigation:
	key: lint-rules/a11y/useAltText
	parent: lint-rules
	title: a11y/useAltText
---

# a11y/useAltText

It asserts that alternative text to images or areas, help to rely on to screen
readers to understand the purpose and the context of the image.

<!-- GENERATED:START(hash:dea426df5ef5b92530aa28617cfcdd9ac7b9a305,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [alt-text](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/alt-text.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f142450e31b7db3f6f4d1115d2e2d2b2c2750ce3,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;presentation&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;presentation&quot;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;none&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;none&quot;</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">object</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">object</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">area</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">area</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.jsx:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    &lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;presentation&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;presentation&quot;</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;none&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;none&quot;</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">object</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">object</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">object</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">object</span><span class="token punctuation">&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">area</span>  <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">area</span>  <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">area</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">area</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/useAltText</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;"> text when using </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>area</strong></span><span style="color: Tomato;">, </span><span style="color: Tomato;"><strong>input type=&apos;image&apos;</strong></span><span style="color: Tomato;">, and </span><span style="color: Tomato;"><strong>object</strong></span>
    <span style="color: Tomato;"> elements.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token attr-name">alt</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Meaningful alternative text on elements helps users relying on screen</span>
    <span style="color: rgb(38, 148, 255);">readers to understand content&apos;s purpose within a page.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&quot;Foo eating a sandwich.&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;Foo eating a sandwich.&quot;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">altText</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token punctuation">${</span><span class="token variable">person</span><span class="token punctuation">}</span><span class="token string"> smiling</span><span class="token string">`</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&quot;&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token attr-name">aria-label</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&quot;id1&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span>&gt;Meaningful description&lt;<span class="token operator">/</span><span class="token variable">object</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span>&gt;<span class="token punctuation">{</span><span class="token variable">hello</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token variable">object</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">object</span> <span class="token attr-name">title</span><span class="token operator">=</span><span class="token string">&quot;An object&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&quot;id1&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">area</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&quot;This is descriptive!&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&quot;This is descriptive!&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token attr-name">aria-label</span><span class="token operator">=</span><span class="token string">&quot;foo&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-jsx"><code class="language-jsx">&lt;<span class="token variable">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&quot;image&quot;</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&quot;id1&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">img</span> <span class="token attr-name">src</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token attr-name">alt</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;Foo eating a sandwich.&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">object</span> <span class="token attr-name">aria-label</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">object</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">object</span> <span class="token attr-name">aria-labelledby</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;id1&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">object</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">object</span> <span class="token attr-name">title</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;An object&quot;</span> <span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">object</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">area</span> <span class="token attr-name">aria-label</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">area</span> <span class="token attr-name">aria-labelledby</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;id1&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">area</span> <span class="token attr-name">alt</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;This is descriptive!&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token attr-name">alt</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;This is descriptive!&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token attr-name">aria-label</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">type</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;image&quot;</span> <span class="token attr-name">aria-labelledby</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;id1&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
