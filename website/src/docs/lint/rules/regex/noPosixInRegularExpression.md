---
title: Lint Rule regex/noPosixInRegularExpression
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/regex/noPosixInRegularExpression
	parent: lint-rules
	title: regex/noPosixInRegularExpression
---

# regex/noPosixInRegularExpression

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:7919c5d8407c80789a6b96fa313e0226c395ee2e,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[:alpha:]]/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/regex/noPosixInRegularExpression</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use POSIX character classes and collating sequences.</span>

    <span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[:alpha:]]/</span>
                      <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">This functionality is not supported in JavaScript regular</span>
    <span style="color: rgb(38, 148, 255);">expressions.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[.ch.]]/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/regex/noPosixInRegularExpression</strong> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use POSIX character classes and collating sequences.</span>

    <span class="token keyword">const</span> <span class="token variable">pattern</span> <span class="token operator">=</span> <span class="token regex">/[[.ch.]]/</span>
                      <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">This functionality is not supported in JavaScript regular</span>
    <span style="color: rgb(38, 148, 255);">expressions.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
