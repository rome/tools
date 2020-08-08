---
title: Lint Rule js/noDupeArgs
layout: layouts/rule.liquid
description: disallow duplicate arguments in `function` definitions
eslint-rule: https://eslint.org/docs/rules/no-dupe-args
eleventyNavigation:
	key: lint-rules/js/noDupeArgs
	parent: lint-rules
	title: js/noDupeArgs
---

# js/noDupeArgs

<!-- GENERATED:START(hash:d817854d5450f106152b96f288c9809f8460a95f,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow duplicate arguments in `function` definitions

**ESLint Equivalent:** [no-dupe-args](https://eslint.org/docs/rules/no-dupe-args)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:027b548f7703de7453760770069bf6586d1fc46f,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">hello</span><span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">//</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.js:1:18</span> <strong>lint/js/noDupeArgs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate function arguments. Check the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> argument.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">function</span> <span class="token variable">hello</span><span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>                  <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token comment">//</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">hello</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
	<span class="token comment">//</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.js:1:18</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Argument </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> name clash in strict mode</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">hello</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span>
     <strong> │ </strong>                  <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token comment">//</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">hello</span> <span class="token operator">=</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token comment">//</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.js:1:27</span> <strong>lint/js/noDupeArgs</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate function arguments. Check the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> argument.</span>

  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">hello</span> <span class="token operator">=</span> <span class="token keyword">function</span> <span class="token punctuation">(</span><span class="token variable">a</span><span class="token punctuation">,</span> <span class="token variable">a</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
     <strong> │ </strong>                           <span style="color: Tomato;"><strong>^</strong></span>
  <strong>  2</strong><strong> │ </strong>  <span class="token comment">//</span>
  <strong>  3</strong><strong> │ </strong><span class="token punctuation">}</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span>
<span class="token keyword">function</span> <span class="token variable">bar</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
