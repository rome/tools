---
title: Lint Rule js/sparseArray
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/sparseArray
	parent: lint-rules
	title: js/sparseArray
---

# js/sparseArray

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:f85b9fb73eb6e8b3e17333808763c88ac3977f5b,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:3</span> <strong>lint/js/sparseArray</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>array</strong></span><span style="color: Tomato;"> contains an </span><span style="color: Tomato;"><strong>empty slot</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span>
       <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Sparse arrays without values for some items can lead to confusion.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>undefined</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
