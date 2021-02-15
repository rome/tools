---
title: Lint Rule react/noDanger
layout: layouts/rule.liquid
description: prevent usage of dangerous JSX props
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-danger.md
eleventyNavigation:
	key: lint-rules/react/noDanger
	parent: lint-rules
	title: react/noDanger
---

# react/noDanger

<!-- GENERATED:START(hash:41779e9547aa42842e64832cf731906935cbf1ab,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent usage of dangerous JSX props

**ESLint Equivalent:** [no-danger](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-danger.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2c7e253f2a9a30970fccc0e141867ab8cc8e0989,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span> <span class="token attr-name">dangerouslySetInnerHTML</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;child&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/react/noDanger</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing content using the </span><span style="color: Tomato;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Tomato;"> prop.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Setting content using code can expose users to cross-site scripting</span>
    <span style="color: rgb(38, 148, 255);">(XSS) attacks.</span>

  <strong><span style="color: Orange;">⚠ </span></strong><span style="color: Orange;">This diagnostic refers to a file that does not exist</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">dangerouslySetInnerHTML</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;child&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:28</span> <strong>lint/react/noDanger</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing content using the </span><span style="color: Tomato;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Tomato;"> prop.</span>

    <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">dangerouslySetInnerHTML</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span>
    <span class="token string">&apos;child&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Setting content using code can expose users to cross-site scripting</span>
    <span style="color: rgb(38, 148, 255);">(XSS) attacks.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">dangerouslySetInnerHTML</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;child&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:22</span> <strong>lint/react/noDanger</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing content using the </span><span style="color: Tomato;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Tomato;"> prop.</span>

    <span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">dangerouslySetInnerHTML</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token variable">__html</span><span class="token punctuation">:</span> <span class="token string">&apos;child&apos;</span> <span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
                          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Setting content using code can expose users to cross-site scripting</span>
    <span style="color: rgb(38, 148, 255);">(XSS) attacks.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">div</span>&gt;Hello World&lt;<span class="token operator">/</span><span class="token variable">div</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token variable">child</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token variable">child</span><span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
