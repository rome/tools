---
title: Lint Rule js/useSingleCaseStatement
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useSingleCaseStatement
	parent: lint-rules
	title: js/useSingleCaseStatement
---

# js/useSingleCaseStatement

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2b509f1b9eacd19bb7808ef11b0ef18638ce3968,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token boolean">true</span><span class="token punctuation">:</span>
	<span class="token keyword">case</span> <span class="token boolean">false</span><span class="token punctuation">:</span>
		<span class="token function">let</span> <span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">;</span>
		<span class="token function">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:1</span> <strong>lint/js/useSingleCaseStatement</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">A switch case should only have a single statement. If you want more,</span>
    <span style="color: Tomato;">then wrap it in a block.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token boolean">true</span><span class="token punctuation">:</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token keyword">case</span> <span class="token boolean">false</span><span class="token punctuation">:</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    <span class="token function">let</span> <span class="token function">foo</span> <span class="token operator">=</span> <span class="token string">&apos;&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>    <span class="token function">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">case</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">false:</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">case</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">false:</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>{</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>let<span style="opacity: 0.8;">&middot;</span>foo<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>&quot;&quot;;
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: Tomato;">foo;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: MediumSeaGreen;">foo;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token boolean">true</span><span class="token punctuation">:</span>
	<span class="token keyword">case</span> <span class="token boolean">false</span><span class="token punctuation">:</span>
		<span class="token string">&apos;yes&apos;</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token boolean">true</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
		<span class="token comment">// empty</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">switch</span> <span class="token punctuation">(</span><span class="token function">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">case</span> <span class="token boolean">true</span><span class="token punctuation">:</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
