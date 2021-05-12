---
title: Lint Rule react/noChildrenProp
layout: layouts/rule.liquid
description: prevent passing of children as props
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-children-prop.md
eleventyNavigation:
	key: lint-rules/react/noChildrenProp
	parent: lint-rules
	title: react/noChildrenProp
---

# react/noChildrenProp

<!-- GENERATED:START(hash:468db80ba3e79c90d80fc354fa88067bfb7f8b47,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent passing of children as props

**ESLint Equivalent:** [no-children-prop](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-children-prop.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:0a3af2301dca84099c33f722afd0e593ab5aa0d2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">MyComponent</span> <span class="token attr-name">children</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">MyComponent</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:13</span> <strong>lint/react/noChildrenProp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop.</span>

    &lt;<span class="token variable">MyComponent</span> <span class="token attr-name">children</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">MyComponent</span>&gt;
                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">The canonical way to pass children in React is to use JSX elements or</span>
    <span style="color: rgb(38, 148, 255);">additional arguments to React.createElement.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">children</span><span class="token punctuation">:</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:28</span> <strong>lint/react/noChildrenProp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop.</span>

    <span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">children</span><span class="token punctuation">:</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
                                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">The canonical way to pass children in React is to use JSX elements or</span>
    <span style="color: rgb(38, 148, 255);">additional arguments to React.createElement.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx"><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">children</span><span class="token punctuation">:</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:22</span> <strong>lint/react/noChildrenProp</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop.</span>

    <span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token variable">children</span><span class="token punctuation">:</span> <span class="token string">&apos;foo&apos;</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
                          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">The canonical way to pass children in React is to use JSX elements or</span>
    <span style="color: rgb(38, 148, 255);">additional arguments to React.createElement.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">MyComponent</span>&gt;&lt;<span class="token variable">AnotherComponent</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">MyComponent</span>  &gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token string">&apos;children&apos;</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx"><span class="token variable">React</span><span class="token punctuation">.</span><span class="token function">createElement</span><span class="token punctuation">(</span><span class="token string">&apos;div&apos;</span><span class="token punctuation">,</span> <span class="token variable">child1</span><span class="token punctuation">,</span> <span class="token string">&apos;child2&apos;</span><span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
