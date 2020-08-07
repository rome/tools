---
title: Lint Rule js/noLabelVar
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noLabelVar
	parent: lint-rules
	title: js/noLabelVar
---

# js/noLabelVar

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:fb746f999e0c7bb36df71e8c11559777815d992f,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span>
<span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">expr</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noLabelVar</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use the x variable name as a label.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">expr</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Creating a label with the same name as an in-scope variable leads to</span>
    <span style="color: DodgerBlue;">confusion.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span>
<span class="token variable">z</span><span class="token punctuation">:</span> <span class="token variable">expr</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
