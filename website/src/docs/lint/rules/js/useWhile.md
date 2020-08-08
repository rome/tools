---
title: Lint Rule js/useWhile
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useWhile
	parent: lint-rules
	title: js/useWhile
---

# js/useWhile

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:8f354e6aadf9681b631b3bb5fcc7e679193772ae,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token function">x</span><span class="token punctuation">.</span><span class="token function">running</span><span class="token punctuation">;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <span class="token function">x</span><span class="token punctuation">.</span><span class="token function">step</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useWhile</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>while</strong></span><span style="color: Tomato;"> loops instead of </span><span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"> loops.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span> <span class="token function">x</span><span class="token punctuation">.</span><span class="token function">running</span><span class="token punctuation">;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token function">x</span><span class="token punctuation">.</span><span class="token function">step</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">x.running</span><span style="color: Tomato;"><strong>;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">{</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>while</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(x.running)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">{</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>x.step();

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span><span class="token punctuation">;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <span class="token function">doSomething</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useWhile</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>while</strong></span><span style="color: Tomato;"> loops instead of </span><span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"> loops.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span><span class="token punctuation">;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token function">doSomething</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>;;</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">{</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>while</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>true</strong></span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">{</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>doSomething();

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
