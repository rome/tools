---
title: Lint Rule js/noArguments
layout: layouts/rule.liquid
description: disallow the use of `arguments`
eslint-rule: https://eslint.org/docs/rules/prefer-rest-params
eleventyNavigation:
	key: lint-rules/js/noArguments
	parent: lint-rules
	title: js/noArguments
---

# js/noArguments

<!-- GENERATED:START(hash:44c761235e8a5357412cfd6b76a049f77107ec6a,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow the use of `arguments`

**ESLint Equivalent:** [prefer-rest-params](https://eslint.org/docs/rules/prefer-rest-params)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:4b08477456818e445044952fa637776ec69603ec,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token function">f</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:13</span> <strong>lint/js/noArguments</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token function">f</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Arguments does not have Array.prototype methods and can be</span>
    <span style="color: DodgerBlue;">inconvenient to use.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:13</span> <strong>lint/js/noArguments</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token punctuation">(</span><span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Arguments does not have Array.prototype methods and can be</span>
    <span style="color: DodgerBlue;">inconvenient to use.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">C</span> <span class="token punctuation">{</span>
	<span class="token function">fn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:14</span> <strong>lint/js/noArguments</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">class</span> <span class="token variable">C</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">fn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Arguments does not have Array.prototype methods and can be</span>
    <span style="color: DodgerBlue;">inconvenient to use.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">o</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
	<span class="token function">fn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:3:14</span> <strong>lint/js/noArguments</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">o</span> <span class="token operator">=</span> <span class="token punctuation">{</span>
  <strong>  2</strong><strong> │ </strong>  <span class="token function">fn</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>    <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">arguments</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
     <strong> │ </strong>                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  4</strong><strong> │ </strong>  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <strong>  5</strong><strong> │ </strong><span class="token punctuation">}</span><span class="token punctuation">;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Arguments does not have Array.prototype methods and can be</span>
    <span style="color: DodgerBlue;">inconvenient to use.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
