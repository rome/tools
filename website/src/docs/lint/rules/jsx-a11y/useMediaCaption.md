---
title: Lint Rule jsx-a11y/useMediaCaption
layout: layouts/rule.liquid
description: enforces that `audio` and `video` elements must have a `track` for captions
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/media-has-caption.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/useMediaCaption
	parent: lint-rules
	title: jsx-a11y/useMediaCaption
---

# jsx-a11y/useMediaCaption

<!-- GENERATED:START(hash:72046e7e1ff3ae410f0415b78503d0fe0f599bcf,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforces that `audio` and `video` elements must have a `track` for captions

**ESLint Equivalent:** [media-has-caption](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/media-has-caption.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:edebc9b4f7efce3e0c127a0bd8be9e4d40f717ac,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">audio</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useMediaCaption</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>track</strong></span><span style="color: Tomato;"> for captions when using </span><span style="color: Tomato;"><strong>audio</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>video</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">audio</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Captions support users with hearing-impairments. They should be a</span>
    <span style="color: DodgerBlue;">transcription or translation of the dialogue, sound effects, musica</span>l
    <span style="color: DodgerBlue;">cues, and other relevant audio information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">video</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useMediaCaption</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>track</strong></span><span style="color: Tomato;"> for captions when using </span><span style="color: Tomato;"><strong>audio</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>video</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">video</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Captions support users with hearing-impairments. They should be a</span>
    <span style="color: DodgerBlue;">transcription or translation of the dialogue, sound effects, musica</span>l
    <span style="color: DodgerBlue;">cues, and other relevant audio information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">audio</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">audio</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useMediaCaption</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>track</strong></span><span style="color: Tomato;"> for captions when using </span><span style="color: Tomato;"><strong>audio</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>video</strong></span><span style="color: Tomato;"> elements.</span>

    &lt;<span class="token attr-name">audio</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">audio</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Captions support users with hearing-impairments. They should be a</span>
    <span style="color: DodgerBlue;">transcription or translation of the dialogue, sound effects, musica</span>l
    <span style="color: DodgerBlue;">cues, and other relevant audio information.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">video</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">audio</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:1</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Expected a corresponding JSX closing tag for </span><span style="color: Tomato;"><strong>video</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Originated from opening tag of </span><span style="color: DodgerBlue;"><strong>video</strong></span>

    &lt;<span class="token attr-name">video</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">audio</span>&gt;
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">audio</span>&gt;&lt;<span class="token attr-name">track</span> <span class="token attr-name">kind</span><span class="token operator">=</span><span class="token string">&apos;captions&apos;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">audio</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">video</span>&gt;&lt;<span class="token attr-name">track</span> <span class="token attr-name">kind</span><span class="token operator">=</span><span class="token string">&apos;captions&apos;</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">video</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">video</span> <span class="token attr-name">muted</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">video</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
