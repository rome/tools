---
title: Lint Rule js/useBlockStatements
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useBlockStatements
	parent: lint-rules
	title: js/useBlockStatements
---

# js/useBlockStatements

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:6b26ccb5ec366cfb89e010b8a708b0bc3aa9d1e2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token function">x</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token function">x</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">if</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(x)</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">x;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">if</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(x)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: MediumSeaGreen;">x;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">x</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token function">y</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token function">x</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token function">y</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong>1</strong><strong> │ </strong>  if<span style="opacity: 0.8;">&middot;</span>(x)<span style="opacity: 0.8;">&middot;</span>{
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>x;
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">}</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">else</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">y;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">else</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: MediumSeaGreen;">y;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>5</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token function">x</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">y</span><span class="token punctuation">)</span> <span class="token function">y</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:7</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">x</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token function">y</span><span class="token punctuation">)</span> <span class="token function">y</span><span class="token punctuation">;</span>
     <strong> │ </strong>       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">if</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(y)</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">y;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">if</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(y)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: MediumSeaGreen;">y;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span><span class="token punctuation">;</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token punctuation">;</span><span class="token punctuation">;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">for</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(;;)</span><span style="color: Tomato;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">for</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(;;)</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">p</span> <span class="token keyword">in</span> <span class="token function">obj</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">p</span> <span class="token keyword">in</span> <span class="token function">obj</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">for</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(p</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">in</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">obj)</span><span style="color: Tomato;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">for</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(p</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">in</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">obj)</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">x</span> <span class="token function">of</span> <span class="token function">xs</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">x</span> <span class="token function">of</span> <span class="token function">xs</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">for</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(x</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">of</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">xs)</span><span style="color: Tomato;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">for</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">of</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">xs)</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">do</span><span class="token punctuation">;</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">do</span><span class="token punctuation">;</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">do</span><span style="color: Tomato;"><strong>;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">while</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(x);</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">do</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;">while</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(x);</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useBlockStatements</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Block statements</strong></span><span style="color: Tomato;"> are preferred in this position.</span>

    <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">while</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(x)</span><span style="color: Tomato;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">while</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(x)</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">with</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">&apos;with&apos; in strict mode</span>

    <span class="token keyword">with</span> <span class="token punctuation">(</span><span class="token function">x</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
