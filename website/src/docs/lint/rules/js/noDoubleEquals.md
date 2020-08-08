---
title: Lint Rule js/noDoubleEquals
layout: layouts/rule.liquid
description: require the use of `===` and `!==`
eslint-rule: https://eslint.org/docs/rules/eqeqeq
eleventyNavigation:
	key: lint-rules/js/noDoubleEquals
	parent: lint-rules
	title: js/noDoubleEquals
---

# js/noDoubleEquals

<!-- GENERATED:START(hash:2b57f2ecfefec5bdcfc5d391cf5a86bc7f54d181,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
require the use of `===` and `!==`

**ESLint Equivalent:** [eqeqeq](https://eslint.org/docs/rules/eqeqeq)
<!-- GENERATED:END(id:description) -->

It is generally bad practice to use `==` for comparison instead of `===`. Double operators will triger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion) and are thus not prefered. Using strict equality operators is almost always best practice.

For ergonomic reasons, this rule makes an exception for `== null` for comparing to both `null` and `undefined`.

<!-- GENERATED:START(hash:63978d7a04d73b528b3f9cfe7dedc896ba06a9d4,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token operator">==</span> <span class="token variable">bar</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noDoubleEquals</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>==</strong></span><span style="color: Tomato;">.</span>

    <span class="token variable">foo</span> <span class="token operator">==</span> <span class="token variable">bar</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">== is only allowed when comparing against null.</span>

  <strong>Suggested fix:</strong> Use ===

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">foo</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">==</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">bar</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">==</span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">bar</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This may be unsafe if you are relying on type coercion</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token operator">==</span> <span class="token boolean">null</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token operator">!=</span> <span class="token boolean">null</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token boolean">null</span> <span class="token operator">==</span> <span class="token variable">foo</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token boolean">null</span> <span class="token operator">!=</span> <span class="token variable">foo</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
