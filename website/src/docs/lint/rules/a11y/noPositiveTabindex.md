---
title: Lint Rule a11y/noPositiveTabindex
layout: layouts/rule.liquid
description: enforce tabIndex value is not greater than zero
eslint-rule: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/tabindex-no-positive.md
eleventyNavigation:
	key: lint-rules/a11y/noPositiveTabindex
	parent: lint-rules
	title: a11y/noPositiveTabindex
---

# a11y/noPositiveTabindex

<!-- GENERATED:START(hash:3f7e6881b133b33804d9c1814858e7b1e06668d5,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce tabIndex value is not greater than zero

**ESLint Equivalent:** [tabindex-no-positive](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/master/docs/rules/tabindex-no-positive.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:056e984870f6478b303ca1a6f9369f7473cbb21b,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;1&quot;</span><span class="token punctuation">&gt;</span>foo<span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1:6</span> <strong>lint/a11y/noPositiveTabindex</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid positive integer values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> attribute.</span>

    <span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;1&quot;</span><span class="token punctuation">&gt;</span>foo<span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span>
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Elements with a positive tab index override natural page content</span>
    <span style="color: rgb(38, 148, 255);">order. This causes elements without a positive tab index to come last</span>
    <span style="color: rgb(38, 148, 255);">when navigating using a keyboard.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;span</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>tabindex=&quot;1&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;span&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>foo
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  &lt;/span&gt;

</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;5&quot;</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/noPositiveTabindex</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid positive integer values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;5&quot;</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Elements with a positive tab index override natural page content</span>
    <span style="color: rgb(38, 148, 255);">order. This causes elements without a positive tab index to come last</span>
    <span style="color: rgb(38, 148, 255);">when navigating using a keyboard.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;span</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>tabIndex=&quot;5&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;span&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>foo
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  &lt;/span&gt;

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">5</span><span class="token punctuation">}</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/noPositiveTabindex</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid positive integer values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">5</span><span class="token punctuation">}</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Elements with a positive tab index override natural page content</span>
    <span style="color: rgb(38, 148, 255);">order. This causes elements without a positive tab index to come last</span>
    <span style="color: rgb(38, 148, 255);">when navigating using a keyboard.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;span</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>tabIndex={5}</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;span&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>foo
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  &lt;/span&gt;

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;5&quot;</span><span class="token punctuation">}</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/a11y/noPositiveTabindex</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid positive integer values for the </span><span style="color: Tomato;"><strong>tabIndex</strong></span><span style="color: Tomato;"> attribute.</span>

    &lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;5&quot;</span><span class="token punctuation">}</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Elements with a positive tab index override natural page content</span>
    <span style="color: rgb(38, 148, 255);">order. This causes elements without a positive tab index to come last</span>
    <span style="color: rgb(38, 148, 255);">when navigating using a keyboard.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;span</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>tabIndex={&quot;5&quot;}</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;span&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>foo
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  &lt;/span&gt;

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;0&quot;</span><span class="token punctuation">&gt;</span>baz<span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">span</span> <span class="token attr-name">tabindex</span><span class="token attr-equals">=</span><span class="token attr-value">&quot;-1&quot;</span><span class="token punctuation">&gt;</span>baz<span class="token punctuation">&lt;/</span><span class="token attr-name">span</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token number">0</span><span class="token punctuation">}</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">}</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;-1&quot;</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token string">&quot;0&quot;</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">dynamic</span><span class="token punctuation">}</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">span</span> <span class="token attr-name">tabIndex</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">undefined</span><span class="token punctuation">}</span>&gt;baz&lt;<span class="token operator">/</span><span class="token variable">span</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
