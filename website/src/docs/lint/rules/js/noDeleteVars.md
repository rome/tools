---
title: Lint Rule js/noDeleteVars
layout: layouts/rule.liquid
description: disallow deleting variables
eslint-rule: https://eslint.org/docs/rules/no-delete-var
eleventyNavigation:
	key: lint-rules/js/noDeleteVars
	parent: lint-rules
	title: js/noDeleteVars
---

# js/noDeleteVars

<!-- GENERATED:START(hash:9b88984d9547056bca3bdfb12a20d79f662fff28,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow deleting variables

**ESLint Equivalent:** [no-delete-var](https://eslint.org/docs/rules/no-delete-var)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:5b064c11c2f028eb4899dad0593e52a066caafbb,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span>
<span class="token keyword">delete</span> <span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.js:2</span> <strong>lint/js/noDeleteVars</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This is an invalid use of the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token keyword">delete</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Only object properties can be deleted.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">arr</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">[</span><span class="token string">&apos;a&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;b&apos;</span><span class="token punctuation">,</span><span class="token string">&apos;c&apos;</span><span class="token punctuation">]</span><span class="token punctuation">,</span> <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">,</span> <span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
<span class="token keyword">delete</span> <span class="token variable">arr</span><span class="token punctuation">[</span><span class="token number">0</span><span class="token punctuation">]</span><span class="token punctuation">[</span><span class="token number">2</span><span class="token punctuation">]</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">obj</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">a</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">b</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token variable">c</span><span class="token punctuation">:</span> <span class="token number">123</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
<span class="token keyword">delete</span> <span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">a</span><span class="token punctuation">.</span><span class="token variable">b</span><span class="token punctuation">.</span><span class="token variable">c</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token keyword">new</span> <span class="token variable">Set</span><span class="token punctuation">(</span><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">,</span><span class="token number">3</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
<span class="token variable">foo</span><span class="token punctuation">.</span><span class="token keyword">delete</span><span class="token punctuation">(</span><span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
