---
title: Lint Rule js/noSingleCharRegexAlternatives
layout: layouts/rule.liquid
description: disallow the use of single character alternations in regular expressions
eleventyNavigation:
	key: lint-rules/js/noSingleCharRegexAlternatives
	parent: lint-rules
	title: js/noSingleCharRegexAlternatives
---

# js/noSingleCharRegexAlternatives

<!-- GENERATED:START(hash:b1523944176bf0cc6a29b4983c98b0729d9742c4,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow the use of single character alternations in regular expressions
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:f28627073d9e961eeeab0a345c9a73a8dc928a14,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/a|b/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1:1</span> <strong>lint/js/noSingleCharRegexAlternatives</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">No single character alternations in regular expressions. Use a</span>
    <span style="color: Tomato;">character class instead.</span>

    <span class="token regex">/a|b/</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">a</span><span style="color: Tomato;"><strong>|</strong></span><span style="color: Tomato;">b</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>[</strong></span><span style="color: MediumSeaGreen;">ab</span><span style="color: MediumSeaGreen;"><strong>]</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/a|b|c/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1:1</span> <strong>lint/js/noSingleCharRegexAlternatives</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">No single character alternations in regular expressions. Use a</span>
    <span style="color: Tomato;">character class instead.</span>

    <span class="token regex">/a|b|c/</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">a</span><span style="color: Tomato;"><strong>|</strong></span><span style="color: Tomato;">b</span><span style="color: Tomato;"><strong>|</strong></span><span style="color: Tomato;">c</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>[</strong></span><span style="color: MediumSeaGreen;">abc</span><span style="color: MediumSeaGreen;"><strong>]</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/[ab]/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/ab|ba/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/a/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/ab|ba|a/</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
