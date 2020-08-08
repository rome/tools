---
title: Lint Rule js/noDebugger
layout: layouts/rule.liquid
description: disallow the use of `debugger`
eslint-rule: https://eslint.org/docs/rules/no-debugger
eleventyNavigation:
	key: lint-rules/js/noDebugger
	parent: lint-rules
	title: js/noDebugger
---

# js/noDebugger

<!-- GENERATED:START(hash:3907daf480d3f0eae86ba6bc2a886339f62699d3,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow the use of `debugger`

**ESLint Equivalent:** [no-debugger](https://eslint.org/docs/rules/no-debugger)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:52d477da6933fdb483566ea334bad95cc4de6c6c,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">debugger</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noDebugger</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>debugger</strong></span><span style="color: Tomato;"> statement.</span>

    <span class="token keyword">debugger</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>debugger;</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">test</span> <span class="token operator">=</span> <span class="token punctuation">{</span> <span class="token keyword">debugger</span><span class="token punctuation">:</span> <span class="token number">1</span> <span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token variable">test</span><span class="token punctuation">.</span><span class="token keyword">debugger</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
