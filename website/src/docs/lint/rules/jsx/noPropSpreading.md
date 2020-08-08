---
title: Lint Rule jsx/noPropSpreading
layout: layouts/rule.liquid
description: prevent JSX prop spreading
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-props-no-spreading.md
eleventyNavigation:
	key: lint-rules/jsx/noPropSpreading
	parent: lint-rules
	title: jsx/noPropSpreading
---

# jsx/noPropSpreading

<!-- GENERATED:START(hash:9fef4e0b18bfd6188de2d0d0e92a4fbe7011fb56,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent JSX prop spreading

**ESLint Equivalent:** [jsx-props-no-spreading](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-props-no-spreading.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:3114bf1603b02063701d92ad1110eafeacd34ccf,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">App</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx/noPropSpreading</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using property spreading in JSX components.</span>

    &lt;<span class="token attr-name">App</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Explicit JSX attributes enhance the readability of code by clearly</span>
    <span style="color: DodgerBlue;">indicating which props are accepted by a given element.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">MyCustomComponent</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">some_other_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">some_other_prop</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:19</span> <strong>lint/jsx/noPropSpreading</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using property spreading in JSX components.</span>

    &lt;<span class="token attr-name">MyCustomComponent</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token attr-name">some_other_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">some_other_prop</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
                       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Explicit JSX attributes enhance the readability of code by clearly</span>
    <span style="color: DodgerBlue;">indicating which props are accepted by a given element.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">MyCustomComponent</span> <span class="token attr-name">some_other_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">some_other_prop</span><span class="token punctuation">}</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:53</span> <strong>lint/jsx/noPropSpreading</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using property spreading in JSX components.</span>

    &lt;<span class="token attr-name">MyCustomComponent</span> <span class="token attr-name">some_other_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">some_other_prop</span><span class="token punctuation">}</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
                                                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Explicit JSX attributes enhance the readability of code by clearly</span>
    <span style="color: DodgerBlue;">indicating which props are accepted by a given element.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx/noPropSpreading</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using property spreading in JSX components.</span>

    &lt;<span class="token attr-name">img</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Explicit JSX attributes enhance the readability of code by clearly</span>
    <span style="color: DodgerBlue;">indicating which props are accepted by a given element.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">MyCustomComponent</span> <span class="token attr-name">one_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">one_prop</span><span class="token punctuation">}</span> <span class="token attr-name">two_prop</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">two_prop</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">src</span><span class="token punctuation">}</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">alt</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
