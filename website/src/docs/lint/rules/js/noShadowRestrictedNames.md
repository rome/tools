---
title: Lint Rule js/noShadowRestrictedNames
layout: layouts/rule.liquid
description: disallow identifiers from shadowing restricted names
eslint-rule: https://eslint.org/docs/rules/no-shadow-restricted-names
eleventyNavigation:
	key: lint-rules/js/noShadowRestrictedNames
	parent: lint-rules
	title: js/noShadowRestrictedNames
---

# js/noShadowRestrictedNames

<!-- GENERATED:START(hash:1245e3f2f132c0183f0a1a01845c052e1b76d7ff,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow identifiers from shadowing restricted names

**ESLint Equivalent:** [no-shadow-restricted-names](https://eslint.org/docs/rules/no-shadow-restricted-names)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:44d887a7eecb7450e7569ab7a731e1cf1a283158,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">NaN</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:9</span> <strong>lint/js/noShadowRestrictedNames</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not shadow the global </span><span style="color: Tomato;"><strong>NaN</strong></span><span style="color: Tomato;"> property.</span>

    <span class="token keyword">function</span> <span class="token variable">NaN</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider renaming this variable. It&apos;s easy to confuse the origin of</span>
    <span style="color: DodgerBlue;">variables when they&apos;re named after a known global.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">Set</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noShadowRestrictedNames</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not shadow the global </span><span style="color: Tomato;"><strong>Set</strong></span><span style="color: Tomato;"> property.</span>

    <span class="token keyword">let</span> <span class="token variable">Set</span><span class="token punctuation">;</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider renaming this variable. It&apos;s easy to confuse the origin of</span>
    <span style="color: DodgerBlue;">variables when they&apos;re named after a known global.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span>  <span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">Object</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:15</span> <strong>lint/js/noShadowRestrictedNames</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not shadow the global </span><span style="color: Tomato;"><strong>Object</strong></span><span style="color: Tomato;"> property.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span>  <span class="token punctuation">}</span> <span class="token keyword">catch</span><span class="token punctuation">(</span><span class="token variable">Object</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
                   <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider renaming this variable. It&apos;s easy to confuse the origin of</span>
    <span style="color: DodgerBlue;">variables when they&apos;re named after a known global.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token operator">!</span><span class="token keyword">function</span> <span class="token variable">Array</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:10</span> <strong>lint/js/noShadowRestrictedNames</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not shadow the global </span><span style="color: Tomato;"><strong>Array</strong></span><span style="color: Tomato;"> property.</span>

    <span class="token operator">!</span><span class="token keyword">function</span> <span class="token variable">Array</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider renaming this variable. It&apos;s easy to confuse the origin of</span>
    <span style="color: DodgerBlue;">variables when they&apos;re named after a known global.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">test</span><span class="token punctuation">(</span><span class="token variable">JSON</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">JSON</span><span class="token punctuation">)</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:14</span> <strong>lint/js/noShadowRestrictedNames</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not shadow the global </span><span style="color: Tomato;"><strong>JSON</strong></span><span style="color: Tomato;"> property.</span>

    <span class="token keyword">function</span> <span class="token variable">test</span><span class="token punctuation">(</span><span class="token variable">JSON</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">JSON</span><span class="token punctuation">)</span><span class="token punctuation">}</span>
                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider renaming this variable. It&apos;s easy to confuse the origin of</span>
    <span style="color: DodgerBlue;">variables when they&apos;re named after a known global.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
