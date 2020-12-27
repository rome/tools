---
title: Lint Rule html/useValidLang
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/html/useValidLang
	parent: lint-rules
	title: html/useValidLang
---

# html/useValidLang

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:e75a56fcf65c837ad96c6c3261b757370be235fd,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;ex&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;ex&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo-bar&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo-bar&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;aa-zz&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;aa-zz&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Did you mean </span><span style="color: DodgerBlue;"><strong>aa-AF</strong></span><span style="color: DodgerBlue;">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aa-</span><span style="color: Tomato;"><strong>zz</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aa-</span><span style="color: MediumSeaGreen;"><strong>AF</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Or one of these?</span>

  <span style="opacity: 0.8;">- </span>aa-AL
  <span style="opacity: 0.8;">- </span>aa-DZ
  <span style="opacity: 0.8;">- </span>aa-AS
  <span style="opacity: 0.8;">- </span>aa-AD
  <span style="opacity: 0.8;">- </span>aa-AO
  <span style="opacity: 0.8;">- </span>aa-AI
  <span style="opacity: 0.8;">- </span>aa-AQ
  <span style="opacity: 0.8;">- </span>aa-AG
  <span style="opacity: 0.8;">- </span>aa-AR
  <span style="opacity: 0.8;">- </span>aa-AM
  <span style="opacity: 0.8;">and </span><span style="opacity: 0.8;">222</span><span style="opacity: 0.8;"> others...</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;zz-AA&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;zz-AA&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Did you mean </span><span style="color: DodgerBlue;"><strong>az-AF</strong></span><span style="color: DodgerBlue;">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>z</strong></span><span style="color: Tomato;">z-A</span><span style="color: Tomato;"><strong>A</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;">z-A</span><span style="color: MediumSeaGreen;"><strong>F</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Or one of these?</span>

  <span style="opacity: 0.8;">- </span>az-AL
  <span style="opacity: 0.8;">- </span>az-AS
  <span style="opacity: 0.8;">- </span>az-AD
  <span style="opacity: 0.8;">- </span>az-AO
  <span style="opacity: 0.8;">- </span>az-AI
  <span style="opacity: 0.8;">- </span>az-AQ
  <span style="opacity: 0.8;">- </span>az-AG
  <span style="opacity: 0.8;">- </span>az-AR
  <span style="opacity: 0.8;">- </span>az-AM
  <span style="opacity: 0.8;">- </span>az-AW
  <span style="opacity: 0.8;">and </span><span style="opacity: 0.8;">37</span><span style="opacity: 0.8;"> others...</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en2&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/html/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en2&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en-US&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
