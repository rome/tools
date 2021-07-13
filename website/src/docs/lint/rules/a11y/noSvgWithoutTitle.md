---
title: Lint Rule a11y/noSvgWithoutTitle
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/a11y/noSvgWithoutTitle
	parent: lint-rules
	title: a11y/noSvgWithoutTitle
---

# a11y/noSvgWithoutTitle

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:1530245355f936f36147eacaa7be76f772a20821,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/a11y/noSvgWithoutTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>

    &lt;<span class="token variable">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">svg</span>&gt;
	&lt;<span class="token variable">title</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
	&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/a11y/noSvgWithoutTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong>&lt;<span class="token variable">svg</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  &lt;<span class="token variable">title</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  &lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>foo<span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/noSvgWithoutTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>

    <span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>foo<span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">title</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">title</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
<span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.html:1</span> <strong>lint/a11y/noSvgWithoutTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>  <span class="token punctuation">&lt;</span><span class="token tag">title</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">title</span><span class="token punctuation">&gt;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>  <span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong><span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">svg</span>&gt;
	&lt;<span class="token variable">rect</span> <span class="token operator">/</span>&gt;
	&lt;<span class="token variable">rect</span> <span class="token operator">/</span>&gt;
	&lt;<span class="token variable">g</span>&gt;
		&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token variable">g</span>&gt;
			&lt;<span class="token variable">title</span>&gt;
				Pass
			&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
			&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
			&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token operator">/</span><span class="token variable">g</span>&gt;
	&lt;<span class="token operator">/</span><span class="token variable">g</span>&gt;
&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-tsx"><code class="language-tsx">&lt;<span class="token variable">svg</span>&gt;
	&lt;<span class="token variable">title</span>&gt;Pass&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
	&lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">rect</span> <span class="token punctuation">/&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">rect</span> <span class="token punctuation">/&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">g</span><span class="token punctuation">&gt;</span>
		<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
		<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
		<span class="token punctuation">&lt;</span><span class="token tag">g</span><span class="token punctuation">&gt;</span>
			<span class="token punctuation">&lt;</span><span class="token tag">title</span><span class="token punctuation">&gt;</span>
				Pass
			<span class="token punctuation">&lt;/</span><span class="token attr-name">title</span><span class="token punctuation">&gt;</span>
			<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
			<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
		<span class="token punctuation">&lt;/</span><span class="token attr-name">g</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;/</span><span class="token attr-name">g</span><span class="token punctuation">&gt;</span>
<span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">svg</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">title</span><span class="token punctuation">&gt;</span>Pass<span class="token punctuation">&lt;/</span><span class="token attr-name">title</span><span class="token punctuation">&gt;</span>
	<span class="token punctuation">&lt;</span><span class="token tag">circle</span> <span class="token punctuation">/&gt;</span>
<span class="token punctuation">&lt;/</span><span class="token attr-name">svg</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
