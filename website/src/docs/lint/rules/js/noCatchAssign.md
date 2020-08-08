---
title: Lint Rule js/noCatchAssign
layout: layouts/rule.liquid
description: disallow reassigning exceptions in `catch` clauses
eslint-rule: https://eslint.org/docs/rules/no-ex-assign
eleventyNavigation:
	key: lint-rules/js/noCatchAssign
	parent: lint-rules
	title: js/noCatchAssign
---

# js/noCatchAssign

<!-- GENERATED:START(hash:55dff6f591d7ea7ce51f29a057dc19def1b503ee,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow reassigning exceptions in `catch` clauses

**ESLint Equivalent:** [no-ex-assign](https://eslint.org/docs/rules/no-ex-assign)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:1565bb8588f1b022bcb91df7a2273505c5a5b5d5,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">e</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">e</span><span class="token punctuation">;</span> <span class="token variable">e</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:23</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">e</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">e</span><span class="token punctuation">;</span> <span class="token variable">e</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                           <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;test&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:42</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">console</span><span class="token punctuation">.</span><span class="token variable">log</span><span class="token punctuation">(</span><span class="token string">&apos;test&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">[</span><span class="token variable">ex</span><span class="token punctuation">,</span> <span class="token variable">test</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:22</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">[</span><span class="token variable">ex</span><span class="token punctuation">,</span> <span class="token variable">test</span><span class="token punctuation">]</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">message</span><span class="token punctuation">,</span> <span class="token variable">name</span><span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">message</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span> <span class="token variable">name</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:34</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">message</span><span class="token punctuation">,</span> <span class="token variable">name</span><span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">message</span> <span class="token operator">=</span> <span class="token string">&apos;test&apos;</span><span class="token punctuation">;</span> <span class="token variable">name</span> <span class="token operator">=</span> <span class="token number">10</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                      <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:52</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:26</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">let</span> <span class="token variable">a</span><span class="token punctuation">;</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:37</span> <strong>lint/js/noCatchAssign</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not </span><span style="color: Tomato;"><strong>reassign catch parameters</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">ex</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">let</span> <span class="token variable">a</span><span class="token punctuation">;</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">x</span><span class="token punctuation">:</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token variable">ex</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">}</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Use a local variable instead.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">e</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">three</span> <span class="token operator">=</span> <span class="token number">2</span> <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token punctuation">{</span><span class="token variable">e</span><span class="token punctuation">}</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">something</span> <span class="token operator">=</span> <span class="token number">2</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">try</span> <span class="token punctuation">{</span> <span class="token punctuation">}</span> <span class="token keyword">catch</span> <span class="token punctuation">(</span><span class="token variable">e</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token keyword">return</span> <span class="token boolean">false</span><span class="token punctuation">;</span> <span class="token punctuation">}</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
