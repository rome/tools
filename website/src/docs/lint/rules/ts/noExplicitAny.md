---
title: Lint Rule ts/noExplicitAny
layout: layouts/rule.liquid
description: it bans the use of `any`
eleventyNavigation:
	key: lint-rules/ts/noExplicitAny
	parent: lint-rules
	title: ts/noExplicitAny
---

# ts/noExplicitAny

Using the `any` type defeats the purpose of using TypeScript.

When `any` is used, all compiler type checks around that value are ignored.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:77cabf8bc94ba2f65af55432c3105dcbce4f89bf,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">age</span><span class="token punctuation">:</span> <span class="token variable">any</span> <span class="token operator">=</span> <span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">;</span><span class="token variable">age</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:11</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">const</span> <span class="token variable">age</span><span class="token punctuation">:</span> <span class="token variable">any</span> <span class="token operator">=</span> <span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">;</span><span class="token variable">age</span><span class="token punctuation">;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">ages</span><span class="token punctuation">:</span>  <span class="token variable">any</span><span class="token punctuation">[</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">]</span><span class="token punctuation">;</span><span class="token variable">ages</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">const</span> <span class="token variable">ages</span><span class="token punctuation">:</span>  <span class="token variable">any</span><span class="token punctuation">[</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">]</span><span class="token punctuation">;</span><span class="token variable">ages</span><span class="token punctuation">;</span>
                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">ages</span><span class="token punctuation">:</span>  <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">]</span><span class="token punctuation">;</span><span class="token variable">ages</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:19</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">const</span> <span class="token variable">ages</span><span class="token punctuation">:</span>  <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token string">&apos;seventeen&apos;</span><span class="token punctuation">]</span><span class="token punctuation">;</span><span class="token variable">ages</span><span class="token punctuation">;</span>
                       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">any</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">any</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">any</span><span class="token punctuation">[</span><span class="token punctuation">]</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">any</span><span class="token punctuation">[</span><span class="token punctuation">]</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:24</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:30</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:28</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
                                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:28</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span>
    <span class="token function">  </span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:41</span> <strong>lint/ts/noExplicitAny</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>any</strong></span><span style="color: Tomato;"> type.</span>

    <span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">any</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span>
    <span class="token function">  </span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using nonspecific types defeats the purpose of using TypeScript.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">age</span><span class="token punctuation">:</span> <span class="token variable">number</span> <span class="token operator">=</span> <span class="token number">17</span><span class="token punctuation">;</span><span class="token variable">age</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">const</span> <span class="token variable">ages</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">number</span><span class="token operator">&gt;</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token number">17</span><span class="token punctuation">]</span><span class="token punctuation">;</span><span class="token variable">ages</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">string</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token keyword">function</span> <span class="token function">greet</span><span class="token punctuation">(</span><span class="token variable">param</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span><span class="token punctuation">)</span><span class="token punctuation">:</span> <span class="token variable">Array</span><span class="token operator">&lt;</span><span class="token variable">string</span><span class="token operator">&gt;</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token variable">param</span><span class="token punctuation">;</span> <span class="token punctuation">}</span><span class="token punctuation">;</span><span class="token function">greet</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
