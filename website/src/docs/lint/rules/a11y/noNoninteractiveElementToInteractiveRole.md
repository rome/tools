---
title: Lint Rule a11y/noNoninteractiveElementToInteractiveRole
layout: layouts/rule.liquid
description: non-interactive elements should not be assigned interactive roles
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-noninteractive-element-to-interactive-role.md
eleventyNavigation:
	key: lint-rules/a11y/noNoninteractiveElementToInteractiveRole
	parent: lint-rules
	title: a11y/noNoninteractiveElementToInteractiveRole
---

# a11y/noNoninteractiveElementToInteractiveRole

<!-- GENERATED:START(hash:3908576fa556cf0a0a48b5a2d878d36c0abc6769,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
non-interactive elements should not be assigned interactive roles

**ESLint Equivalent:** [no-noninteractive-element-to-interactive-role](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-noninteractive-element-to-interactive-role.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:88aa07898a529d9e05935e30ff8f79c39f5c1dcb,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;checkbox&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/a11y/noNoninteractiveElementToInteractiveRole</strong> ━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> is non-interactive and should not have an</span>
    <span style="color: Tomato;">interactive role.</span>

    &lt;<span class="token variable">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;checkbox&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>h1</strong></span><span style="color: rgb(38, 148, 255);"> with a div or a span.</span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">h1</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;checkbox&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">h1</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:4</span> <strong>lint/a11y/noNoninteractiveElementToInteractiveRole</strong> ━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> is non-interactive and should not have an</span>
    <span style="color: Tomato;">interactive role.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">h1</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;checkbox&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">h1</span><span class="token punctuation">&gt;</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>h1</strong></span><span style="color: rgb(38, 148, 255);"> with a div or a span.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&quot;article&quot;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">h1</span> <span class="token attr-name">role</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;article&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">h1</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
