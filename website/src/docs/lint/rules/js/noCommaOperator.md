---
title: Lint Rule js/noCommaOperator
layout: layouts/rule.liquid
description: disallow comma operators
eslint-rule: https://eslint.org/docs/rules/no-sequences
eleventyNavigation:
	key: lint-rules/js/noCommaOperator
	parent: lint-rules
	title: js/noCommaOperator
---

# js/noCommaOperator

<!-- GENERATED:START(hash:5ff9a5b3890a1303b5d2430c36fe699cfc8ca580,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow comma operators

**ESLint Equivalent:** [no-sequences](https://eslint.org/docs/rules/no-sequences)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:0c357fe564a947b4ea4e057e984e9104e7415de2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">(</span><span class="token number">0</span><span class="token punctuation">,</span> <span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:1</span> <strong>lint/js/noCommaOperator</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Avoid the comma operator</strong></span><span style="color: Tomato;">. It can lead to easy mistakes and ambiguous</span>
    <span style="color: Tomato;">code.</span>

    <span class="token punctuation">(</span><span class="token number">0</span><span class="token punctuation">,</span> <span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">)</span>
     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">If you want multiple expressions, then break it up.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token function">rome</span><span class="token punctuation">(</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noCommaOperator</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Avoid the comma operator</strong></span><span style="color: Tomato;">. It can lead to easy mistakes and ambiguous</span>
    <span style="color: Tomato;">code.</span>

    <span class="token function">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">,</span> <span class="token function">rome</span><span class="token punctuation">(</span><span class="token punctuation">)</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">If you want multiple expressions, then break it up.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token function">foo</span><span class="token punctuation">(</span><span class="token number">0</span><span class="token punctuation">,</span> <span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span><span class="token punctuation">]</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token number">3</span><span class="token punctuation">]</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">b</span><span class="token punctuation">,</span> <span class="token variable">c</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
