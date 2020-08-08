---
title: Lint Rule js/noUndeclaredVariables
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
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

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:399164492231f62d06dc9f09847f117b40fa6aa4,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
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
