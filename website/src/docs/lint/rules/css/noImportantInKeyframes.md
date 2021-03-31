---
title: Lint Rule css/noImportantInKeyframes
layout: layouts/rule.liquid
description: disallow `!important` in `@keyframe` rules
stylelint-rule: https://stylelint.io/user-guide/rules/keyframe-declaration-no-important
eleventyNavigation:
	key: lint-rules/css/noImportantInKeyframes
	parent: lint-rules
	title: css/noImportantInKeyframes
---

# css/noImportantInKeyframes

Disallow `!important` in `@keyframe` rules.

<!-- GENERATED:START(hash:0094121294949d6adeae81bbbd4799e0eb090fce,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**stylelint Equivalent:** [keyframe-declaration-no-important](https://stylelint.io/user-guide/rules/keyframe-declaration-no-important)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:3cff6c9a21cfa550f3c7fb5699ade95d8a2fe9b2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">@keyframes foo {
  from {
    opacity: 0;
  }

  to {
    opacity: 1 !important;
  }
}</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.css:7:4</span> <strong>lint/css/noImportantInKeyframes</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>!important</strong></span><span style="color: Tomato;"> within keyframes declarations is completely ignored</span>
    <span style="color: Tomato;">in some browsers.</span>

  <strong>  6</strong><strong> │ </strong>  to {
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 7</strong><strong> │ </strong>    opacity: 1 !important;
     <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  8</strong><strong> │ </strong>  }
  <strong>  9</strong><strong> │ </strong>}

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">@keyframes foo {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
