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

<!-- GENERATED:START(hash:a1039ea3f9ddf1b2da2a99ef671bb8cca08225ba,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token atrule">@keyframes</span> <span class="token string">foo</span> <span class="token punctuation">{</span>
  <span class="token string">from</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
    <span class="token property">width</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>

  <span class="token string">to</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">1</span> !<span class="token string">important</span><span class="token punctuation">;</span>
    <span class="token property">width</span><span class="token punctuation">:</span> 100px !<span class="token string">important</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.css:8:4</span> <strong>lint/css/noImportantInKeyframes</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using </span><span style="color: Tomato;"><strong>!important</strong></span><span style="color: Tomato;"> within keyframes declarations is completely ignored</span>
    <span style="color: Tomato;">in some browsers.</span>

   <strong>  7</strong><strong> │ </strong>  <span class="token string">to</span> <span class="token punctuation">{</span>
   <strong><span style="color: Tomato;">&gt;</span></strong><strong> 8</strong><strong> │ </strong>    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">1</span> !<span class="token string">important</span><span class="token punctuation">;</span>
      <strong> │ </strong>    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
   <strong><span style="color: Tomato;">&gt;</span></strong><strong> 9</strong><strong> │ </strong>    <span class="token property">width</span><span class="token punctuation">:</span> 100px !<span class="token string">important</span><span class="token punctuation">;</span>
      <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  10</strong><strong> │ </strong>  <span class="token punctuation">}</span>
  <strong>  11</strong><strong> │ </strong><span class="token punctuation">}</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong>1</strong><strong> │ </strong>  to<span style="opacity: 0.8;">&middot;</span>{
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: Tomato;">opacity:</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>!important</strong></span><span style="color: Tomato;">;</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: Tomato;">width:</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">100px</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>!important</strong></span><span style="color: Tomato;">;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: MediumSeaGreen;">opacity:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">1;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: MediumSeaGreen;">width:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">100px;</span>
  <strong>  </strong><strong>4</strong><strong> </strong><strong>4</strong><strong> │ </strong>  }

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-css"><code class="language-css"><span class="token atrule">@keyframes</span> <span class="token string">foo</span> <span class="token punctuation">{</span>
  <span class="token string">from</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>

  <span class="token string">to</span> <span class="token punctuation">{</span>
    <span class="token property">opacity</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">;</span>
  <span class="token punctuation">}</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
