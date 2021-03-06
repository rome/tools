---
title: Lint Rule a11y/noAccessKey
layout: layouts/rule.liquid
description: enforce that the `accessKey` prop is not used on any element to avoid complications with keyboard commands used by a screenreader
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-access-key.md
eleventyNavigation:
	key: lint-rules/a11y/noAccessKey
	parent: lint-rules
	title: a11y/noAccessKey
---

# a11y/noAccessKey

<!-- GENERATED:START(hash:cf22957d9c98e2deeba712dd80a806bd49423e12,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce that the `accessKey` prop is not used on any element to avoid complications with keyboard commands used by a screenreader

**ESLint Equivalent:** [no-access-key](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-access-key.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f49d1c23db59a5b4e31785b71bd10bb1a5f45ea1,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span>&apos;key /&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:17</span> <strong>parse(html)</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The name of the attribute is not valid and should be wrapped in</span>
    <span style="color: Tomato;">double quotes.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span>&apos;key /&gt;
                     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;key&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/noAccessKey</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between</span>
    <span style="color: Tomato;">keyboard shortcuts and screen reader keyboard comments.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;key&quot;</span> <span class="token punctuation">/&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Assigning keyboard shortcuts using the accessKey attribute leads to</span>
    <span style="color: rgb(38, 148, 255);">inconsistent keyboard actions across applications.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
