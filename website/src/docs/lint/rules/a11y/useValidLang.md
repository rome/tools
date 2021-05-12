---
title: Lint Rule a11y/useValidLang
layout: layouts/rule.liquid
description: check if `lang` attribute is valid
eleventyNavigation:
	key: lint-rules/a11y/useValidLang
	parent: lint-rules
	title: a11y/useValidLang
---

# a11y/useValidLang

This rule makes sure the value of `lang` attribute is valid.

This rule is applied to HTML and JSX files.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:163421f62aaa1e7158a7d1dee1c7e87afcb609f5,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;ex&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;ex&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo-bar&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;foo-bar&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;aa-zz&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;aa-zz&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Did you mean </span><span style="color: rgb(38, 148, 255);"><strong>aa-AF</strong></span><span style="color: rgb(38, 148, 255);">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aa-</span><span style="color: Tomato;"><strong>zz</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aa-</span><span style="color: MediumSeaGreen;"><strong>AF</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Or one of these?</span>

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

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;zz-AA&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;zz-AA&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Did you mean </span><span style="color: rgb(38, 148, 255);"><strong>az-AF</strong></span><span style="color: rgb(38, 148, 255);">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>z</strong></span><span style="color: Tomato;">z-A</span><span style="color: Tomato;"><strong>A</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;">z-A</span><span style="color: MediumSeaGreen;"><strong>F</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Or one of these?</span>

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

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en2&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:11</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en2&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;ex&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;ex&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;foo-bar&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;foo-bar&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;aa-zz&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;aa-zz&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Did you mean </span><span style="color: rgb(38, 148, 255);"><strong>aa-AF</strong></span><span style="color: rgb(38, 148, 255);">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">aa-</span><span style="color: Tomato;"><strong>zz</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">aa-</span><span style="color: MediumSeaGreen;"><strong>AF</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Or one of these?</span>

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

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;zz-AA&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/useValidLang</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;zz-AA&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Did you mean </span><span style="color: rgb(38, 148, 255);"><strong>az-AF</strong></span><span style="color: rgb(38, 148, 255);">?</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>z</strong></span><span style="color: Tomato;">z-A</span><span style="color: Tomato;"><strong>A</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;">z-A</span><span style="color: MediumSeaGreen;"><strong>F</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Or one of these?</span>

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

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;en2&gt;&lt;/html&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:23</span> <strong>parse(js)</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unterminated string constant</span>

    &lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;en2&gt;&lt;/html&gt;</span>
                           <span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en-US&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">html</span> <span class="token attr-name">lang</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;en&quot;</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">html</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;en-US&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token string">&apos;en&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">html</span> <span class="token attr-name">lang</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">lang</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">html</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
