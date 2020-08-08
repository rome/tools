---
title: Lint Rule js/noDelete
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noDelete
	parent: lint-rules
	title: js/noDelete
---

# js/noDelete

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:68bf5e84d806698e71c6b07a555c0f2224130d68,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">arr</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">[</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;b&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;c&apos;</span><span class="token punctuation">]</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span> <span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
<span class="token keyword">delete</span> <span class="token variable">arr</span><span class="token punctuation">[</span><span class="token number">0</span><span class="token punctuation">]</span><span class="token punctuation">[</span><span class="token number">2</span><span class="token punctuation">]</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noDelete</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">arr</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">[</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;b&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;c&apos;</span><span class="token punctuation">]</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span> <span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">delete</span> <span class="token variable">arr</span><span class="token punctuation">[</span><span class="token number">0</span><span class="token punctuation">]</span><span class="token punctuation">[</span><span class="token number">2</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">arr[0][2]</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">arr[0][2]</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>undefined</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">obj</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">a</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">b</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">c</span><span class="token punctuation">:</span> <span class="token number">123</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token keyword">delete</span> <span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">a</span><span class="token punctuation">.</span><span class="token variable">b</span><span class="token punctuation">.</span><span class="token variable">c</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noDelete</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">obj</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">a</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">b</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">c</span><span class="token punctuation">:</span> <span class="token number">123</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">delete</span> <span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">a</span><span class="token punctuation">.</span><span class="token variable">b</span><span class="token punctuation">.</span><span class="token variable">c</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">obj.a.b.c</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">obj.a.b.c</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>=</strong></span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>undefined</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token keyword">new</span> <span class="token variable">Set</span><span class="token punctuation">(</span><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">,</span><span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token variable">foo</span><span class="token punctuation">.</span><span class="token keyword">delete</span><span class="token punctuation">(</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
