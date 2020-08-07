---
title: Lint Rule jsx-a11y/noNoninteractiveElementToInteractiveRole
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx-a11y/noNoninteractiveElementToInteractiveRole
	parent: lint-rules
	title: jsx-a11y/noNoninteractiveElementToInteractiveRole
---

# jsx-a11y/noNoninteractiveElementToInteractiveRole

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:27795817dcb0a8bf8c26a4e1ff7891c321273f90,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noNoninteractiveElementToInteractiveRole</strong> ━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The HTML element </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> is non-interactive and should not have an</span>
    <span style="color: Tomato;">interactive role.</span>

    &lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Replace </span><span style="color: DodgerBlue;"><strong>h1</strong></span><span style="color: DodgerBlue;"> with a div or a span.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
