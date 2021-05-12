---
title: Lint Rule js/noSingleVarDeclarator
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eslint-rule: https://eslint.org/docs/rules/one-var
eleventyNavigation:
	key: lint-rules/js/noSingleVarDeclarator
	parent: lint-rules
	title: js/noSingleVarDeclarator
---

# js/noSingleVarDeclarator

<!-- GENERATED:START(hash:49a6abd5645f6016dfc0deaf228ff48edfaa8379,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION

**ESLint Equivalent:** [one-var](https://eslint.org/docs/rules/one-var)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:7e843be0d7cbe2a8c22a0c11daa8c06b4fbd8ff7,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useSingleVarDeclarator</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Declare variables separately.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">bar</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">let</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">bar;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">let</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>let</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">bar;</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">,</span> <span class="token variable">x</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token variable">arr</span><span class="token punctuation">.</span><span class="token variable">length</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
