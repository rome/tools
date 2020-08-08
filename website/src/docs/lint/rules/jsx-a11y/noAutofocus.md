---
title: Lint Rule jsx-a11y/noAutofocus
layout: layouts/rule.liquid
description: discourage the usage of `autoFocus`
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-autofocus.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noAutofocus
	parent: lint-rules
	title: jsx-a11y/noAutofocus
---

# jsx-a11y/noAutofocus

It discourages the usage to the attribute `autoFocus` on elements. This practice could cause issues for
 sighted and non-sighted users.

# Accessibility guidelines

General best practice (reference resources)

### Resources

- [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
- [The accessibility of HTML 5 autofocus](https://www.brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)

<!-- GENERATED:START(hash:f1f0a43a550c358678f238a06775e505d6569006,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [no-autofocus](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-autofocus.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:8275f6449bd0c79f5e6ab69880dc39c9a89e1c11,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noAutofocus</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Automatically focusing elements overrides natural page content focus</span>
    <span style="color: DodgerBlue;">order, causing issues for keyboard-only navigation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noAutofocus</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Automatically focusing elements overrides natural page content focus</span>
    <span style="color: DodgerBlue;">order, causing issues for keyboard-only navigation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>autoFocus=&quot;true&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;false&quot;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noAutofocus</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>autoFocus</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;false&quot;</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Automatically focusing elements overrides natural page content focus</span>
    <span style="color: DodgerBlue;">order, causing issues for keyboard-only navigation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>autoFocus={&quot;false&quot;}</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">autoFocus</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
