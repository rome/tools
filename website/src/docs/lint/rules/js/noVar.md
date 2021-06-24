---
title: Lint Rule js/noVar
layout: layouts/rule.liquid
description: require `let` or `const` instead of `var`
eslint-rule: https://eslint.org/docs/rules/no-var
eleventyNavigation:
	key: lint-rules/js/noVar
	parent: lint-rules
	title: js/noVar
---

# js/noVar

<!-- GENERATED:START(hash:011cdf7622213580aa078f6c2c0dcc4f2a66990e,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
require `let` or `const` instead of `var`

**ESLint Equivalent:** [no-var](https://eslint.org/docs/rules/no-var)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:48813dbfee55193933522f5b4eac26c93d0e029d,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">var</span> <span class="token variable">foobar</span><span class="token punctuation">;</span>
<span class="token variable">foobar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noVar</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Variable declarations using </span><span style="color: Tomato;"><strong>var</strong></span><span style="color: Tomato;"> are disallowed.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">var</span> <span class="token variable">foobar</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token variable">foobar</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Use let or const instead.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
