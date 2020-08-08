---
title: Lint Rule js/noUnsafeNegation
layout: layouts/rule.liquid
description: disallow negating the left operand of relational operators
eslint-rule: https://eslint.org/docs/rules/no-unsafe-negation
eleventyNavigation:
	key: lint-rules/js/noUnsafeNegation
	parent: lint-rules
	title: js/noUnsafeNegation
---

# js/noUnsafeNegation

<!-- GENERATED:START(hash:5215bc12ff2807b3be0976d25a70376a95a1dc29,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow negating the left operand of relational operators

**ESLint Equivalent:** [no-unsafe-negation](https://eslint.org/docs/rules/no-unsafe-negation)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2d5f49ab5a4ad0b49bde184146b678f8db38839b,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token operator">!</span><span class="token number">1</span> <span class="token keyword">in</span> <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noUnsafeNegation</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>negation operator is used unsafely</strong></span><span style="color: Tomato;"> on the left side of this</span>
    <span style="color: Tomato;">binary expression.</span>

    <span class="token operator">!</span><span class="token number">1</span> <span class="token keyword">in</span> <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">!1</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">in</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">[1,</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">2]</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">!</span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">in</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">[1,</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">2]</span><span style="color: MediumSeaGreen;"><strong>)</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
