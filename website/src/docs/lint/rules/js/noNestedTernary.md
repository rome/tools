---
title: Lint Rule js/noNestedTernary
layout: layouts/rule.liquid
description: disallow nested ternary expressions
eslint-rule: https://eslint.org/docs/rules/no-nested-ternary
eleventyNavigation:
	key: lint-rules/js/noNestedTernary
	parent: lint-rules
	title: js/noNestedTernary
---

# js/noNestedTernary

<!-- GENERATED:START(hash:b51cb067c6275fd70c48b0d57e09a5e0b3d18a33,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow nested ternary expressions

**ESLint Equivalent:** [no-nested-ternary](https://eslint.org/docs/rules/no-nested-ternary)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:82a5d21545ab5a3eeacfb9c08de15671ccad9174,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token variable">baz</span> <span class="token operator">===</span> <span class="token variable">qux</span> <span class="token punctuation">?</span> <span class="token variable">quxx</span> <span class="token punctuation">:</span> <span class="token variable">foobar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:24</span> <strong>lint/js/noNestedTernary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Nesting ternary expressions can make code more difficult to</span>
    <span style="color: Tomato;">understand.</span>

    <span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token variable">baz</span> <span class="token operator">===</span> <span class="token variable">qux</span> <span class="token punctuation">?</span> <span class="token variable">quxx</span> <span class="token punctuation">:</span> <span class="token variable">foobar</span><span class="token punctuation">;</span>
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">?</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token punctuation">:</span> <span class="token variable">baz</span> <span class="token punctuation">?</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:36</span> <strong>lint/js/noNestedTernary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Nesting ternary expressions can make code more difficult to</span>
    <span style="color: Tomato;">understand.</span>

    <span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">?</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token punctuation">:</span> <span class="token variable">baz</span> <span class="token punctuation">?</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
                                        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:18</span> <strong>lint/js/noNestedTernary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Nesting ternary expressions can make code more difficult to</span>
    <span style="color: Tomato;">understand.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">baz</span> <span class="token operator">===</span> <span class="token variable">qux</span> <span class="token punctuation">?</span> <span class="token function">quxx</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token function">foobar</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token function">bar</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:6</span> <strong>lint/js/noNestedTernary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Nesting ternary expressions can make code more difficult to</span>
    <span style="color: Tomato;">understand.</span>

    <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">baz</span> <span class="token operator">===</span> <span class="token variable">qux</span> <span class="token punctuation">?</span> <span class="token function">quxx</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token function">foobar</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">:</span> <span class="token function">bar</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token punctuation">:</span> <span class="token variable">foobar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token operator">||</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">||</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token operator">||</span> <span class="token variable">baz</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token operator">||</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">||</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">baz</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token operator">||</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foo</span> <span class="token operator">||</span> <span class="token variable">baz</span> <span class="token punctuation">?</span> <span class="token variable">bar</span> <span class="token operator">||</span> <span class="token variable">boo</span> <span class="token punctuation">:</span> <span class="token variable">foo</span> <span class="token operator">&amp;&amp;</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">bar</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">baz</span> <span class="token operator">===</span> <span class="token variable">qux</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">quxx</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span>
	<span class="token variable">thing</span> <span class="token operator">=</span> <span class="token variable">foobar</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
