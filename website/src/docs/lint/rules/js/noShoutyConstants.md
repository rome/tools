---
title: Lint Rule js/noShoutyConstants
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noShoutyConstants
	parent: lint-rules
	title: js/noShoutyConstants
---

# js/noShoutyConstants

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:8e0d0afe072f95e7b9ec67909788b3ef445ce0c3,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:12</span> <strong>lint/js/noShoutyConstants</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Redundant constant reference</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">You should avoid declaring constants with a string that&apos;s the same</span>
    <span style="color: DodgerBlue;">value as the variable name. It introduces a level of unnecessar</span>y
    <span style="color: DodgerBlue;">indirection when it&apos;s only two additional characters to inline.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This constant is declared here</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">FOO</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">FOO</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
<span class="token keyword">function</span> <span class="token variable">f</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">FOO</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:8</span> <strong>lint/js/noShoutyConstants</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Redundant constant reference</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">You should avoid declaring constants with a string that&apos;s the same</span>
    <span style="color: DodgerBlue;">value as the variable name. It introduces a level of unnecessar</span>y
    <span style="color: DodgerBlue;">indirection when it&apos;s only two additional characters to inline.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This constant is declared here</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">f</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>  <span class="token keyword">return</span> <span class="token variable">FOO</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">FOO</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">FOO</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">,</span> <span class="token variable">BAR</span> <span class="token operator">=</span> <span class="token string">&quot;bar&quot;</span><span class="token punctuation">;</span>
<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:12</span> <strong>lint/js/noShoutyConstants</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Redundant constant reference</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">You should avoid declaring constants with a string that&apos;s the same</span>
    <span style="color: DodgerBlue;">value as the variable name. It introduces a level of unnecessar</span>y
    <span style="color: DodgerBlue;">indirection when it&apos;s only two additional characters to inline.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This constant is declared here</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">,</span> <span class="token variable">BAR</span> <span class="token operator">=</span> <span class="token string">&quot;bar&quot;</span><span class="token punctuation">;</span>
     <strong> │ </strong>      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">FOO</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">FOO</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">const</span> <span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">;</span>
<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">FOO</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">f</span><span class="token punctuation">(</span><span class="token variable">FOO</span> <span class="token operator">=</span> <span class="token string">&quot;FOO&quot;</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> <span class="token variable">FOO</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
