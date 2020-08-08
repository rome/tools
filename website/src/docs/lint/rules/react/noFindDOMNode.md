---
title: Lint Rule react/noFindDOMNode
layout: layouts/rule.liquid
description: prevent usage of `findDOMNode`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-find-dom-node.md
eleventyNavigation:
	key: lint-rules/react/noFindDOMNode
	parent: lint-rules
	title: react/noFindDOMNode
---

# react/noFindDOMNode

<!-- GENERATED:START(hash:b5bf239b2848ca5df01841e78fbeddb8d2344375,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
prevent usage of `findDOMNode`

**ESLint Equivalent:** [no-find-dom-node](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/no-find-dom-node.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:2cd21d9e53c7c5ce73b1dd126cc050e722ccfbda,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">findDOMNode</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">scrollIntoView</span><span class="token punctuation">(</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noFindDOMNode</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>findDOMNode</strong></span><span style="color: Tomato;"> function.</span>

    <span class="token function">findDOMNode</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">scrollIntoView</span><span class="token punctuation">(</span><span class="token punctuation">)</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React plans to deprecate the findDOMNode function entirely since it</span>
    <span style="color: DodgerBlue;">prevents internal optimizations. Use callback refs instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">ReactDOM</span><span class="token punctuation">.</span><span class="token variable">findDOMNode</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token variable">scrollIntoView</span><span class="token punctuation">(</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noFindDOMNode</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using the </span><span style="color: Tomato;"><strong>findDOMNode</strong></span><span style="color: Tomato;"> function.</span>

    <span class="token variable">ReactDOM</span><span class="token punctuation">.</span><span class="token variable">findDOMNode</span><span class="token punctuation">(</span><span class="token keyword">this</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token variable">scrollIntoView</span><span class="token punctuation">(</span><span class="token punctuation">)</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">React plans to deprecate the findDOMNode function entirely since it</span>
    <span style="color: DodgerBlue;">prevents internal optimizations. Use callback refs instead.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">node</span><span class="token punctuation">.</span><span class="token variable">scrollIntoView</span><span class="token punctuation">(</span><span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
