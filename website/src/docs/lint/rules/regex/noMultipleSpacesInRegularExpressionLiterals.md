---
title: Lint Rule regex/noMultipleSpacesInRegularExpressionLiterals
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/regex/noMultipleSpacesInRegularExpressionLiterals
	parent: lint-rules
	title: regex/noMultipleSpacesInRegularExpressionLiterals
---

# regex/noMultipleSpacesInRegularExpressionLiterals

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:c8ecbe5a1fae2ad35b453acb61dac50fafad6fe6,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/   /</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/   /</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {3}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{3}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/  foo/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/  foo/</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {2}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">foo</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{2}</strong></span><span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo   /</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/foo   /</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {3}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">foo</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{3}</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo  bar/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/foo  bar/</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {2}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">foo</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">bar</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{2}</strong></span><span style="color: MediumSeaGreen;">bar</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo   bar    baz/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/foo   bar    baz/</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {7}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">foo</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">bar</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">baz</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{3}</strong></span><span style="color: MediumSeaGreen;">bar</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{4}</strong></span><span style="color: MediumSeaGreen;">baz</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo [ba]r  b(a|z)/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:10</span> <strong>lint/regex/noMultipleSpacesInRegularExpressionLiterals</strong>
<span style="color: #000; background-color: #ddd;">FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>regular expression</strong></span><span style="color: Tomato;"> contains unclear uses of </span><span style="color: Tomato;"><strong>multiple spaces</strong></span><span style="color: Tomato;">.</span>

    <span class="token regex">/foo [ba]r  b(a|z)/</span>
              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">It&apos;s hard to visually count the amount of spaces, it&apos;s clearer if you</span>
    <span style="color: DodgerBlue;">use a quantifier instead. eg / {2}/</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">foo</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">[ba]r</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">b(a|z)</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">[ba]r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;"><strong>{2}</strong></span><span style="color: MediumSeaGreen;">b(a|z)</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo {2}bar/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo bar baz/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token regex">/foo /</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
