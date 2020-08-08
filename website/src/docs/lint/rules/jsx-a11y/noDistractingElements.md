---
title: Lint Rule jsx-a11y/noDistractingElements
layout: layouts/rule.liquid
description: enforce distracting elements are not used
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-distracting-elements.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noDistractingElements
	parent: lint-rules
	title: jsx-a11y/noDistractingElements
---

# jsx-a11y/noDistractingElements

<!-- GENERATED:START(hash:a7b38b5eb09839f5285348000ab4147d0401816d,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce distracting elements are not used

**ESLint Equivalent:** [no-distracting-elements](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-distracting-elements.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:0fba8ff32e7cfee7eaa0b962494f0decd3557422,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">blink</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noDistractingElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using deprecated blink elements.</span>

    &lt;<span class="token attr-name">blink</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Deprecated blink are difficult to read and distract attention away</span>
    <span style="color: DodgerBlue;">from page content, especially for users with visual impairments.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">marquee</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/noDistractingElements</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using deprecated marquee elements.</span>

    &lt;<span class="token attr-name">marquee</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Deprecated marquee are difficult to read and distract attention away</span>
    <span style="color: DodgerBlue;">from page content, especially for users with visual impairments.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
