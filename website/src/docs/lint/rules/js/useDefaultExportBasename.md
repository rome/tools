---
title: Lint Rule js/useDefaultExportBasename
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useDefaultExportBasename
	parent: lint-rules
	title: js/useDefaultExportBasename
---

# js/useDefaultExportBasename

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:3dadeb39a682f94bb29b9c5aad7d6fbae431665f,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token function">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">foo.ts:1:24</span> <strong>lint/js/useDefaultExportBasename</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The filename and the name of a default function should match.</span>

    <span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token function">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">The filename should be </span><span style="color: rgb(38, 148, 255);"><strong>test.ts</strong></span><span style="color: rgb(38, 148, 255);"> or the function name should be </span><span style="color: rgb(38, 148, 255);"><strong>foo</strong></span><span style="color: rgb(38, 148, 255);">.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">foo.ts:1:21</span> <strong>lint/js/useDefaultExportBasename</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The filename and the name of a default class should match.</span>

    <span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">The filename should be </span><span style="color: rgb(38, 148, 255);"><strong>Test.ts</strong></span><span style="color: rgb(38, 148, 255);"> or the class name should be </span><span style="color: rgb(38, 148, 255);"><strong>Foo</strong></span><span style="color: rgb(38, 148, 255);">.</span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token function">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token string">&apos;rome&apos;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
