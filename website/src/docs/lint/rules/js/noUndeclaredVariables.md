---
title: Lint Rule js/noUndeclaredVariables
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eslint-rule: https://eslint.org/docs/rules/no-undef
eleventyNavigation:
	key: lint-rules/js/noUndeclaredVariables
	parent: lint-rules
	title: js/noUndeclaredVariables
---

# js/noUndeclaredVariables

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:399164492231f62d06dc9f09847f117b40fa6aa4,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foobar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noUndeclaredVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>foobar</strong></span><span style="color: Tomato;"> variable is undeclared</span>

    <span class="token variable">foobar</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->

<!-- GENERATED:START(hash:5e2d6b27806b51889eb92a8c0ba2ee9db7b1623e,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [no-undef](https://eslint.org/docs/rules/no-undef)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:c0b09a509166e06f0bd34fb6288b0421b4fd3823,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foobar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noUndeclaredVariables</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>foobar</strong></span><span style="color: Tomato;"> variable is undeclared</span>

    <span class="token variable">foobar</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
