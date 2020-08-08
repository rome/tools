---
title: Lint Rule js/noDuplicateImportSource
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noDuplicateImportSource
	parent: lint-rules
	title: js/noDuplicateImportSource
---

# js/noDuplicateImportSource

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:735dc4b177245f40c920be99feeb4f9b4a936c19,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">import</span>	<span class="token variable">foo</span>	<span class="token keyword">from</span>	<span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">import</span>	<span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>	<span class="token keyword">from</span>	<span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">import</span>	<span class="token variable">type</span>	<span class="token punctuation">{</span><span class="token variable">fooType</span><span class="token punctuation">}</span>	<span class="token keyword">from</span>	<span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>

<span class="token keyword">const</span>	<span class="token variable">typedFoo</span><span class="token punctuation">:</span>	<span class="token variable">fooType</span>	<span class="token operator">=</span>	<span class="token punctuation">{</span>
	<span class="token variable">type</span><span class="token punctuation">:</span>	<span class="token string">&apos;foo&apos;</span>
<span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noDuplicateImportSource</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This module has </span><span style="color: Tomato;"><strong>already been imported</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Previously imported here</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">import</span>  <span class="token variable">foo</span>  <span class="token keyword">from</span>  <span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong><span class="token keyword">import</span>  <span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>  <span class="token keyword">from</span>  <span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>
  <strong>  3</strong><strong> │ </strong><span class="token keyword">import</span>  <span class="token variable">type</span>  <span class="token punctuation">{</span><span class="token variable">fooType</span><span class="token punctuation">}</span>  <span class="token keyword">from</span>  <span class="token string">&apos;./testdummy.ts&apos;</span><span class="token punctuation">;</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
