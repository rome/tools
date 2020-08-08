---
title: Lint Rule jsx-a11y/noAccessKey
layout: layouts/rule.liquid
description: enforce that the `accessKey` prop is not used on any element to avoid complications with keyboard commands used by a screenreader
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-access-key.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noAccessKey
	parent: lint-rules
	title: jsx-a11y/noAccessKey
---

# jsx-a11y/noAccessKey

<!-- GENERATED:START(hash:cf22957d9c98e2deeba712dd80a806bd49423e12,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce that the `accessKey` prop is not used on any element to avoid complications with keyboard commands used by a screenreader

**ESLint Equivalent:** [no-access-key](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-access-key.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:29a95e2aa1d053e0f494881fb521ea99cedc3ebb,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token string">&apos;key /&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:24</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unterminated string constant</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token string">&apos;key /&gt;</span>
                            <span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">key</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noAccessKey</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between</span>
    <span style="color: Tomato;">keyboard shortcuts and screen reader keyboard comments.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">key</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Assigning keyboard shortcuts using the accessKey attribute leads to</span>
    <span style="color: DodgerBlue;">inconsistent keyboard actions across applications.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>accessKey={key}</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
