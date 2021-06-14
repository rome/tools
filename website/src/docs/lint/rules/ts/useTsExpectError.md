---
title: Lint Rule ts/useTsExpectError
layout: layouts/rule.liquid
showHero: false
description: use `@ts-expect-error` suppressions instead of `@ts-ignore` to get notified when suppression is no longer necessary
eleventyNavigation:
	key: lint-rules/ts/useTsExpectError
	parent: lint-rules
	title: ts/useTsExpectError
---

# ts/useTsExpectError

Prefer `@ts-expect-error` to get notified when suppression is no longer necessary.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:cded587940dfda865f5cf9296b1f431b9ecd9de2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-ignore</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">// @ts-ignore</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">//</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">//</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-ignore: Blah blah blah</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">// @ts-ignore: Blah blah blah</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">//</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">Blah</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">blah</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">blah</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">//</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">Blah</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">blah</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">blah</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/* @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">/* @ts-ignore */</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">/*</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*/</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">/*</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*/</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/** @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">/** @ts-ignore */</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">/**</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;">*/</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">/**</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or*/</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> * @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">/**</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token comment"> * @ts-ignore */</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong>1</strong><strong> │ </strong>  /**
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*/</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*/</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>4</strong><strong> </strong><strong>4</strong><strong> │ </strong>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> ** @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">filename.ts:1</span> <strong>lint/ts/useTsExpectError</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Prefer @ts-expect-error to get notified when suppression is no longer</span>
    <span style="color: Tomato;">necessary.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token comment">/**</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token comment"> ** @ts-ignore */</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  3</strong><strong> │ </strong><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong>1</strong><strong> │ </strong>  /**
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">**</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">@ts-</span><span style="color: Tomato;"><strong>ign</strong></span><span style="color: Tomato;">or</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">*/</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">**</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">@ts-</span><span style="color: MediumSeaGreen;"><strong>expect-err</strong></span><span style="color: MediumSeaGreen;">or</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">*/</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  let<span style="opacity: 0.8;">&middot;</span>foo:<span style="opacity: 0.8;">&middot;</span>boolean<span style="opacity: 0.8;">&middot;</span>=<span style="opacity: 0.8;">&middot;</span>1;
  <strong>  </strong><strong>4</strong><strong> </strong><strong>4</strong><strong> │ </strong>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-expect-error</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-expect-error: Blah blah blah</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/* @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/** @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> * @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> ** @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
