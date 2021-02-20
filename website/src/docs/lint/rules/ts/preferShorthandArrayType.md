---
title: Lint Rule ts/preferShorthandArrayType
layout: layouts/rule.liquid
description: promotes the use of `[]` over `Array<>`
eleventyNavigation:
	key: lint-rules/ts/preferShorthandArrayType
	parent: lint-rules
	title: ts/preferShorthandArrayType
---

# ts/preferShorthandArrayType

MISSING DOCUMENTATION

When expressing array types, this rule promotes the usage of `[]` shorthand instead
of `Array<>`.

<!-- GENERATED:START(hash:88c7071800ffce914fde4e92ccd215f518d39c1b,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
<span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:11</span> <strong>lint/ts/preferShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
     <strong> │ </strong>           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">bar</span><span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>&gt;</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><strong>[]</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/ts/preferShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">string</span><span style="color: Tomato;"><strong>&gt;&gt;</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><strong>[]</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>|</strong></span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">string</span><span style="color: MediumSeaGreen;"><strong>[])[]</strong></span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:24</span> <strong>lint/ts/preferShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">string</span><span style="color: Tomato;"><strong>&gt;</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">string</span><span style="color: MediumSeaGreen;"><strong>[]</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Promise</span><span class="token operator">&lt;</span><span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:21</span> <strong>lint/ts/preferShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Promise</span><span class="token operator">&lt;</span><span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">string</span><span style="color: Tomato;"><strong>&gt;</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">string</span><span style="color: MediumSeaGreen;"><strong>[]</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Foo</span><span class="token operator">&lt;</span><span class="token variable">Bar</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/ts/preferShorthandArrayType</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">invalid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Foo</span><span class="token operator">&lt;</span><span class="token variable">Bar</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span><span class="token punctuation">;</span>
                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>Array&lt;</strong></span><span style="color: Tomato;">Foo&lt;Bar&gt;</span><span style="color: Tomato;"><strong>&gt;</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">Foo&lt;Bar&gt;</span><span style="color: MediumSeaGreen;"><strong>[]</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">foo</span> <span class="token operator">|</span> <span class="token variable">bar</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Foo</span> <span class="token operator">|</span> <span class="token variable">Bar</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">valid</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">keyof</span> <span class="token variable">Bar</span><span class="token operator">&gt;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->
