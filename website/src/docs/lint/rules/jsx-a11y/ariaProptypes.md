---
title: Lint Rule jsx-a11y/ariaProptypes
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx-a11y/ariaProptypes
	parent: lint-rules
	title: jsx-a11y/ariaProptypes
---

# jsx-a11y/ariaProptypes

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:51c6e2765cc282862b00ed17cd8088750436d0af,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token string">&apos;test&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:22</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-checked</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token string">&apos;test&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
                          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;test&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-autocomplete</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;test&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The supported values for the </span><span style="color: DodgerBlue;"><strong>aria-autocomplete</strong></span><span style="color: DodgerBlue;"> attribute are:</span>
    <span style="color: DodgerBlue;">&quot;inline&quot;, &quot;list&quot;, &quot;both&quot;, &quot;none&quot;</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-invalid</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The supported values for the </span><span style="color: DodgerBlue;"><strong>aria-invalid</strong></span><span style="color: DodgerBlue;"> attribute are: &quot;grammar&quot;,</span>
    <span style="color: DodgerBlue;">false, &quot;spelling&quot;, true</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-errormessage</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-errormessage</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-errormessage</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-relevant</span><span class="token operator">=</span><span class="token string">&apos;fancy&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-relevant</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-relevant</span><span class="token operator">=</span><span class="token string">&apos;fancy&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The supported values for the </span><span style="color: DodgerBlue;"><strong>aria-relevant</strong></span><span style="color: DodgerBlue;"> attribute are:</span>
    <span style="color: DodgerBlue;">&quot;additions&quot;, &quot;all&quot;, &quot;removals&quot;, &quot;text&quot;</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-labelledby</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">`</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-labelledby</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">`</span><span class="token string">`</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-details</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/ariaProptypes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The value of the ARIA attribute </span><span style="color: Tomato;"><strong>aria-details</strong></span><span style="color: Tomato;"> is not correct.</span>

    &lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-details</span><span class="token operator">=</span><span class="token string">&apos;&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">checked</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">true</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token string">&apos;false&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-checked</span><span class="token operator">=</span><span class="token string">&apos;mixed&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;both&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;inline&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;list&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-autocomplete</span><span class="token operator">=</span><span class="token string">&apos;none&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token string">&apos;true&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token string">&apos;grammar&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token string">&apos;false&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-invalid</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token boolean">false</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-errormessage</span><span class="token operator">=</span><span class="token string">&apos;someid&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-relevant</span><span class="token operator">=</span><span class="token string">&apos;additions&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">aria-relevant</span><span class="token operator">=</span><span class="token string">&apos;additions all&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&apos;id&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-labelledby</span><span class="token operator">=</span><span class="token string">&apos;fooId barId&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span> <span class="token attr-name">aria-details</span><span class="token operator">=</span><span class="token string">&apos;someid&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
