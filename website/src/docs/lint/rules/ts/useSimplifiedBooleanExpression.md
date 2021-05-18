---
title: Lint Rule js/useSimplifiedBooleanExpression
layout: layouts/rule.liquid
description: discard redundant terms or operators in boolean expressions
eleventyNavigation:
	key: lint-rules/js/useSimplifiedBooleanExpression
	parent: lint-rules
	title: js/useSimplifiedBooleanExpression
---

# js/useSimplifiedBooleanExpression

<!-- GENERATED:START(hash:3b7ad80345c8a7a3e4f38c5619551b4f6f2bc642,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
discard redundant terms or operators in boolean expressions
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:9ec7acb4dc752fba30da906e26f54e40527b8bd2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:34</span> <strong>lint/ts/useSimplifiedBooleanExpression</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Boolean expression contains unnecessary complexity.</span>

    <span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!!</strong></span><span style="color: Tomato;">x</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">x</span> <span class="token operator">===</span> <span class="token boolean">true</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:34</span> <strong>lint/ts/useSimplifiedBooleanExpression</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Boolean expression contains unnecessary complexity.</span>

    <span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">x</span> <span class="token operator">===</span> <span class="token boolean">true</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">x</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>true</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token boolean">false</span> <span class="token operator">===</span> <span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:34</span> <strong>lint/ts/useSimplifiedBooleanExpression</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Boolean expression contains unnecessary complexity.</span>

    <span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token boolean">false</span> <span class="token operator">===</span> <span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>false</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">x</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>!</strong></span><span style="color: MediumSeaGreen;">x</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">,</span> <span class="token variable">y</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token punctuation">(</span><span class="token variable">x</span> <span class="token operator">||</span> <span class="token variable">y</span><span class="token punctuation">)</span> <span class="token operator">===</span> <span class="token boolean">true</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:46</span> <strong>lint/ts/useSimplifiedBooleanExpression</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Boolean expression contains unnecessary complexity.</span>

    <span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">,</span> <span class="token variable">y</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token punctuation">(</span><span class="token variable">x</span> <span class="token operator">||</span> <span class="token variable">y</span><span class="token punctuation">)</span> <span class="token operator">===</span> <span class="token boolean">true</span> <span class="token punctuation">}</span>
                                                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">||</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">y</span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>true</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">||</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">y</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">?</span><span class="token punctuation">:</span> <span class="token variable">boolean</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token operator">!</span><span class="token operator">!</span><span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">|</span> <span class="token variable">number</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">x</span> <span class="token operator">===</span> <span class="token boolean">true</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">|</span> <span class="token variable">undefined</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token boolean">false</span> <span class="token operator">===</span> <span class="token variable">x</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
