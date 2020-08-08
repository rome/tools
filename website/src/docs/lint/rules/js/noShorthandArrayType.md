---
title: Lint Rule js/noShorthandArrayType
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noShorthandArrayType
	parent: lint-rules
	title: js/noShorthandArrayType
---

# js/noShorthandArrayType

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f33d2aa01ac0206f54694dec0a9c88a2a8f35be5,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
<span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:13</span> <strong>lint/js/noShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
     <strong> │ </strong>             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">bar</span><span style="color: Tomato;"><strong>[]</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>Array&lt;</strong></span><span style="color: MediumSeaGreen;">bar</span><span style="color: MediumSeaGreen;"><strong>&gt;</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
