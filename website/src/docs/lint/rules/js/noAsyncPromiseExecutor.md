---
title: Lint Rule js/noAsyncPromiseExecutor
layout: layouts/rule.liquid
description: disallow using an async function as a Promise executor
eslint-rule: https://eslint.org/docs/rules/no-async-promise-executor
eleventyNavigation:
	key: lint-rules/js/noAsyncPromiseExecutor
	parent: lint-rules
	title: js/noAsyncPromiseExecutor
---

# js/noAsyncPromiseExecutor

<!-- GENERATED:START(hash:8975a4388dcae81640eb8549915683626fac10fa,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow using an async function as a Promise executor

**ESLint Equivalent:** [no-async-promise-executor](https://eslint.org/docs/rules/no-async-promise-executor)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:56da5390cc4504686b46ddd3ad9b0a17380abd2c,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:12</span> <strong>lint/js/noAsyncPromiseExecutor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Promise executor functions</strong></span><span style="color: Tomato;"> should not be </span><span style="color: Tomato;"><strong>async</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This can lead to lost errors and unnecessary indirection.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:12</span> <strong>lint/js/noAsyncPromiseExecutor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Promise executor functions</strong></span><span style="color: Tomato;"> should not be </span><span style="color: Tomato;"><strong>async</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span>
                <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This can lead to lost errors and unnecessary indirection.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:16</span> <strong>lint/js/noAsyncPromiseExecutor</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Promise executor functions</strong></span><span style="color: Tomato;"> should not be </span><span style="color: Tomato;"><strong>async</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">)</span>
                    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">This can lead to lost errors and unnecessary indirection.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">new</span> <span class="token variable">Promise</span><span class="token punctuation">(</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">,</span> <span class="token keyword">async</span> <span class="token keyword">function</span> <span class="token variable">unrelated</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token punctuation">{</span><span class="token punctuation">}</span> <span class="token keyword">new</span> <span class="token variable">Foo</span><span class="token punctuation">(</span><span class="token keyword">async</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
