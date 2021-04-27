---
title: Lint Rule a11y/noAccessKey
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/a11y/noAccessKey
	parent: lint-rules
	title: a11y/noAccessKey
---

# a11y/noAccessKey

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f6746f1439b70de181bef8317d224fbe8724ef97,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token string">&quot;key&quot;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/a11y/noAccessKey</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between</span>
    <span style="color: Tomato;">keyboard shortcuts and screen reader keyboard comments.</span>

    &lt;<span class="token variable">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token string">&quot;key&quot;</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Assigning keyboard shortcuts using the accessKey attribute leads to</span>
    <span style="color: rgb(38, 148, 255);">inconsistent keyboard actions across applications.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>accessKey=&quot;key&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">key</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/a11y/noAccessKey</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between</span>
    <span style="color: Tomato;">keyboard shortcuts and screen reader keyboard comments.</span>

    &lt;<span class="token variable">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">key</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Assigning keyboard shortcuts using the accessKey attribute leads to</span>
    <span style="color: rgb(38, 148, 255);">inconsistent keyboard actions across applications.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>accessKey={key}</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;key&quot;</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:7</span> <strong>lint/a11y/noAccessKey</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between</span>
    <span style="color: Tomato;">keyboard shortcuts and screen reader keyboard comments.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token attr-name">accesskey</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;key&quot;</span> <span class="token punctuation">/&gt;</span>
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Assigning keyboard shortcuts using the accessKey attribute leads to</span>
    <span style="color: rgb(38, 148, 255);">inconsistent keyboard actions across applications.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>accesskey=&quot;key&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">input</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">Input</span> <span class="token attr-name">accessKey</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">key</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
