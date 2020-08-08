---
title: Lint Rule js/noCompareNegZero
layout: layouts/rule.liquid
description: disallow comparing against `-0`
eslint-rule: https://eslint.org/docs/rules/no-compare-neg-zero
eleventyNavigation:
	key: lint-rules/js/noCompareNegZero
	parent: lint-rules
	title: js/noCompareNegZero
---

# js/noCompareNegZero

<!-- GENERATED:START(hash:4e66dc704574da74ac6e7703d67bd9beaab54c5f,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow comparing against `-0`

**ESLint Equivalent:** [no-compare-neg-zero](https://eslint.org/docs/rules/no-compare-neg-zero)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:360419660d9c041cd9b588fa2f896a559b8c557a,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token number">1</span> <span class="token operator">&gt;=</span> <span class="token operator">-</span><span class="token number">0</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noCompareNegZero</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use the </span><span style="color: Tomato;"><strong>&gt;=</strong></span><span style="color: Tomato;"> operator to compare against </span><span style="color: Tomato;"><strong>-0</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">(</span><span class="token number">1</span> <span class="token operator">&gt;=</span> <span class="token operator">-</span><span class="token number">0</span><span class="token punctuation">)</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token number">1</span> <span class="token operator">&gt;=</span> <span class="token number">0</span><span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
