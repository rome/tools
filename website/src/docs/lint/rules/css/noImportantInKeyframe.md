---
title: Lint Rule css/noImportantInKeyframe
layout: layouts/rule.liquid
description: disallow `!important` in `@keyframe` rules
stylelint-rule: https://stylelint.io/user-guide/rules/keyframe-declaration-no-important
eleventyNavigation:
	key: lint-rules/css/noImportantInKeyframe
	parent: lint-rules
	title: css/noImportantInKeyframe
---

# css/noImportantInKeyframe

Disallow `!important` in `@keyframe` rules.

<!-- GENERATED:START(hash:0094121294949d6adeae81bbbd4799e0eb090fce,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**stylelint Equivalent:** [keyframe-declaration-no-important](https://stylelint.io/user-guide/rules/keyframe-declaration-no-important)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:8507aa7f60f9adc190e9c161a3da075f448d1d2e,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text">@keyframes important1 {
   from {
     margin-top: 50px;
   }
   to {
     margin-top: 100px !important;
   }
 }</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.css:6:5</span> <strong>lint/css/noImportantInKeyframe</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>!important</strong></span><span style="color: Tomato;"> within keyframes declarations is completely ignored</span>
    <span style="color: Tomato;">in some browsers.</span>

  <strong>  4</strong><strong> │ </strong>   }
  <strong>  5</strong><strong> │ </strong>   to {
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 6</strong><strong> │ </strong>     margin-top: 100px !important;
     <strong> │ </strong>     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  7</strong><strong> │ </strong>   }
  <strong>  8</strong><strong> │ </strong> }

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text">@keyframes important1 {
  from {
    margin-top: 50px;
  }
  to {
    margin-top: 100px;
  }
}</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
