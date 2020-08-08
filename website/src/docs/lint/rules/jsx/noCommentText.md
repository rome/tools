---
title: Lint Rule jsx/noCommentText
layout: layouts/rule.liquid
description: comments inside children section of tag should be placed inside braces
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-comment-textnodes.md
eleventyNavigation:
	key: lint-rules/jsx/noCommentText
	parent: lint-rules
	title: jsx/noCommentText
---

# jsx/noCommentText

<!-- GENERATED:START(hash:482e522619198fb4afb30bf8decf571d658c9747,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
comments inside children section of tag should be placed inside braces

**ESLint Equivalent:** [jsx-no-comment-textnodes](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-comment-textnodes.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:bf2bd4e2b52d40114743640c1cb2ffb1f5c859ce,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;// comment&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:15</span> <strong>lint/jsx/noCommentText</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;// comment&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
                   <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">JavaScript comment sequences are not supported by JSX and result in</span>
    <span style="color: DodgerBlue;">unwanted characters on-screen.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;">/</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">comment</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><strong>**</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">comment</span><span style="color: MediumSeaGreen;"><strong>*/}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;/* comment */&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:15</span> <strong>lint/jsx/noCommentText</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;/* comment */&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
                   <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">JavaScript comment sequences are not supported by JSX and result in</span>
    <span style="color: DodgerBlue;">unwanted characters on-screen.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">/*</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">comment</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*/</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><strong>*</strong></span><span style="color: MediumSeaGreen;">*</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">comment</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*/</span><span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;/** comment */&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:15</span> <strong>lint/jsx/noCommentText</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;/** comment */&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
                   <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">JavaScript comment sequences are not supported by JSX and result in</span>
    <span style="color: DodgerBlue;">unwanted characters on-screen.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">/**</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">comment</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*/</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;">/**</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">comment</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*/</span><span style="color: MediumSeaGreen;"><strong>}</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token comment">/* comment */</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span>&gt;<span class="token punctuation">{</span><span class="token comment">/** comment */</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">a</span> <span class="token operator">=</span> &lt;<span class="token attr-name">div</span> <span class="token attr-name">className</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token string">&quot;cls&quot;</span> <span class="token comment">/* comment */</span><span class="token punctuation">}</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
