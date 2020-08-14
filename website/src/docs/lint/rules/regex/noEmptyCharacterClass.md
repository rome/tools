---
title: Lint Rule regex/noEmptyCharacterClass
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/regex/noEmptyCharacterClass
	parent: lint-rules
	title: regex/noEmptyCharacterClass
---

# regex/noEmptyCharacterClass

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:29027042ad86aedbf9bc8bb87654dae640d30d52,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/^abc[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:15</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/^abc[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span>
                   <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/foo[]bar/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:14</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/foo[]bar/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span>
                  <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">;</span><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">.</span><span class="token variable">match</span><span class="token punctuation">(</span><span class="token regex">/^abc[]/</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span><span class="token punctuation">;</span> <span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:20</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">;</span><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token variable">foo</span><span class="token punctuation">.</span><span class="token variable">match</span><span class="token punctuation">(</span><span class="token regex">/^abc[]/</span><span class="token punctuation">)</span><span class="token punctuation">)</span> <span class="token punctuation">{</span> <span class="token variable">foo</span><span class="token punctuation">;</span> <span class="token punctuation">}</span>
                        <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[]]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:11</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[]]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span>
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/\[[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:13</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/\[[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span>
                 <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/\[\[\]a-z[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:20</span> <strong>lint/regex/noEmptyCharacterClass</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Do not use </span><span style="color: Tomato;"><strong>empty character classes in regular expressions</strong></span><span style="color: Tomato;">.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/\[\[\]a-z[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span>
                        <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Empty character classes are usually typos.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/^abc[a-zA-Z]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">regExp</span> <span class="token operator">=</span> <span class="token keyword">new</span> <span class="token variable">RegExp</span><span class="token punctuation">(</span><span class="token string">&apos;^abc[]&apos;</span><span class="token punctuation">)</span><span class="token punctuation">;</span><span class="token variable">regExp</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/^abc/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\]]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[a-zA-Z\[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\[a-z[]]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\-\[\]\/\{\}\(\)\*\+\?\.\\^\$\|]/g</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\]]/uy</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/[\]]/s</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span> <span class="token operator">=</span> <span class="token regex">/\[]/</span><span class="token punctuation">;</span><span class="token variable">foo</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
