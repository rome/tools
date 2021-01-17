---
title: Lint Rule jsx-a11y/svgHasTitle
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx-a11y/svgHasTitle
	parent: lint-rules
	title: jsx-a11y/svgHasTitle
---

# jsx-a11y/svgHasTitle

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:d6ec935933391b8e0e4741bad20e956d28d708f6,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/svgHasTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> when using </span><span style="color: Tomato;"><strong>svg</strong></span>

    &lt;<span class="token attr-name">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">For accessibility purposes, SVGs should have a title.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">svg</span>&gt;
        &lt;<span class="token attr-name">title</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">title</span>&gt;
        &lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
    &lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/svgHasTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> when using </span><span style="color: Tomato;"><strong>svg</strong></span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong>&lt;<span class="token attr-name">svg</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>        &lt;<span class="token attr-name">title</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">title</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>        &lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>    &lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">For accessibility purposes, SVGs should have a title.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">svg</span>&gt;
		&lt;<span class="token attr-name">rect</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token attr-name">rect</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token attr-name">g</span>&gt;
			&lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
			&lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
			&lt;<span class="token attr-name">g</span>&gt;
				&lt;<span class="token attr-name">title</span>&gt;
					Pass
				&lt;<span class="token operator">/</span><span class="token attr-name">title</span>&gt;
				&lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
				&lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
			&lt;<span class="token operator">/</span><span class="token attr-name">g</span>&gt;
		&lt;<span class="token operator">/</span><span class="token attr-name">g</span>&gt;
	&lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">svg</span>&gt;
        &lt;<span class="token attr-name">title</span>&gt;Pass&lt;<span class="token operator">/</span><span class="token attr-name">title</span>&gt;
        &lt;<span class="token attr-name">circle</span> <span class="token operator">/</span>&gt;
    &lt;<span class="token operator">/</span><span class="token attr-name">svg</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:d212b39f0f10dd203f4fb4d3083c25cb9d4447d0,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/svgHasTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> when using </span><span style="color: Tomato;"><strong>svg</strong></span>

    &lt;<span class="token variable">svg</span>&gt;foo&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">svg</span>&gt;
    &lt;<span class="token variable">title</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
    &lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:4</span> <strong>lint/jsx-a11y/svgHasTitle</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Alternative text </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> element cannot be empty</span>

  <strong>  1</strong><strong> │ </strong>&lt;<span class="token variable">svg</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>    &lt;<span class="token variable">title</span>&gt;&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong>    &lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
  <strong>  4</strong><strong> │ </strong>&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">For accessibility purposes, </span><span style="color: rgb(38, 148, 255);"><strong>SVGs</strong></span><span style="color: rgb(38, 148, 255);"> should have an alternative text,</span>
    <span style="color: rgb(38, 148, 255);">provided via </span><span style="color: rgb(38, 148, 255);"><strong>title</strong></span><span style="color: rgb(38, 148, 255);"> element.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">svg</span>&gt;
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
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token variable">svg</span>&gt;
    &lt;<span class="token variable">title</span>&gt;Pass&lt;<span class="token operator">/</span><span class="token variable">title</span>&gt;
    &lt;<span class="token variable">circle</span> <span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span><span class="token variable">svg</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
