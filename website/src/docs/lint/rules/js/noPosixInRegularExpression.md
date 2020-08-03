---
title: Lint Rule js/noPosixInRegularExpression
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noPosixInRegularExpression
	parent: lint-rules
	title: js/noPosixInRegularExpression
---

# js/noPosixInRegularExpression

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:2a94939a4d5a972b8a924fba9a2f249a986fd6f1,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[:alpha:]]/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/js/noPosixInRegularExpression</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use POSIX character classes and collating sequences.</span>

    <span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[:alpha:]]/</span>
                      <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This functionality is not supported in JavaScript regular</span>
    <span style="color: DodgerBlue;">expressions.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[.ch.]]/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/js/noPosixInRegularExpression</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use POSIX character classes and collating sequences.</span>

    <span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[.ch.]]/</span>
                      <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This functionality is not supported in JavaScript regular</span>
    <span style="color: DodgerBlue;">expressions.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
