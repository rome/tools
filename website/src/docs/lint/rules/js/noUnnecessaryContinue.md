---
title: Lint Rule js/noUnnecessaryContinue
layout: layouts/rule.liquid
showHero: false
description: disallow unnecessary continue statement inside the loop
eleventyNavigation:
	key: lint-rules/js/noUnnecessaryContinue
	parent: lint-rules
	title: js/noUnnecessaryContinue
---

# js/noUnnecessaryContinue

`continue` statement is unnecessary as the last statement inside the loop, can be safely removed

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:5c7decda24f79ecf3886be7a17b992a516215df4,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">loop</span><span class="token punctuation">:</span> <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">5</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">continue</span> <span class="token variable">loop</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:1</span> <strong>lint/js/noUnnecessaryContinue</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"> statement</span>

  <strong>  1</strong><strong> │ </strong><span class="token variable">loop</span><span class="token punctuation">:</span> <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">5</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">continue</span> <span class="token variable">loop</span><span class="token punctuation">;</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>loop;</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">i</span><span class="token operator">--</span><span class="token punctuation">)</span>
		<span class="token keyword">continue</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:2</span> <strong>lint/js/noUnnecessaryContinue</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"> statement</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">i</span><span class="token operator">--</span><span class="token punctuation">)</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>    <span class="token keyword">continue</span><span class="token punctuation">;</span>
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>continue;</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token number">1</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">continue</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:1</span> <strong>lint/js/noUnnecessaryContinue</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"> statement</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token number">1</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">continue</span><span class="token punctuation">;</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>continue;</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
   	<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">i</span> <span class="token operator">&gt;</span> <span class="token number">5</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
       	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
       	<span class="token keyword">continue</span><span class="token punctuation">;</span>
   	<span class="token punctuation">}</span>
   	<span class="token keyword">else</span> <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">i</span> <span class="token operator">&gt;=</span> <span class="token number">5</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">8</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
       	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;test&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
   	<span class="token punctuation">}</span>
   	<span class="token keyword">else</span> <span class="token punctuation">{</span>
       	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;test&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
   	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:4:8</span> <strong>lint/js/noUnnecessaryContinue</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"> statement</span>

  <strong>  2</strong><strong> │ </strong>     <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">i</span> <span class="token operator">&gt;</span> <span class="token number">5</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong>  3</strong><strong> │ </strong>         <span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>         <span class="token keyword">continue</span><span class="token punctuation">;</span>
     <strong> │ </strong>         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  5</strong><strong> │ </strong>     <span class="token punctuation">}</span>
  <strong>  6</strong><strong> │ </strong>     <span class="token keyword">else</span> <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">i</span> <span class="token operator">&gt;=</span> <span class="token number">5</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">8</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>continue;</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">9</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">continue</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:1</span> <strong>lint/js/noUnnecessaryContinue</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>continue</strong></span><span style="color: Tomato;"> statement</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">9</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token keyword">continue</span><span class="token punctuation">;</span>
     <strong> │ </strong>  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>continue;</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">i</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">i</span> <span class="token operator">&gt;</span> <span class="token number">5</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">continue</span><span class="token punctuation">;</span>
	<span class="token punctuation">}</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">i</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
	<span class="token variable">i</span><span class="token operator">--</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">i</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">continue</span><span class="token punctuation">;</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token variable">i</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token variable">condition</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">conditionZ</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">conditionX</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;log&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
			<span class="token keyword">continue</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
		<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;log&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">conditionY</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token variable">console</span><span class="token punctuation">.</span><span class="token function">log</span><span class="token punctuation">(</span><span class="token string">&apos;log&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">loop</span><span class="token punctuation">:</span> <span class="token keyword">while</span> <span class="token punctuation">(</span><span class="token number">1</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">forLoop</span><span class="token punctuation">:</span> <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">5</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span><span class="token punctuation">(</span><span class="token variable">someCondition</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">continue</span> <span class="token variable">loop</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">loop</span><span class="token punctuation">:</span> <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">i</span> <span class="token operator">&lt;</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token variable">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">let</span> <span class="token variable">j</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> <span class="token variable">j</span> <span class="token operator">&lt;</span> <span class="token variable">byteLength</span><span class="token punctuation">;</span> <span class="token variable">j</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
		<span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">condition</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
			<span class="token keyword">continue</span> <span class="token variable">loop</span><span class="token punctuation">;</span>
		<span class="token punctuation">}</span>
	<span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
