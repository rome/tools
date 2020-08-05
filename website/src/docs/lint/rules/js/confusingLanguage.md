---
title: Lint Rule js/confusingLanguage
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/confusingLanguage
	parent: lint-rules
	title: js/confusingLanguage
---

# js/confusingLanguage

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:77933ff3c7ab3833d4d91f7937f7ab66e30882ae,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">//	the	blacklist</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:5</span> <strong>lint/js/confusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token comment">//</span><span class="token comment">  </span><span class="token comment">the</span><span class="token comment">  </span><span class="token comment">blacklist</span>
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">/*	the</span>
<span class="token comment">blacklist	*/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">blacklist</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">blacklist</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">BLACKLIST</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">someBlacklist</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/confusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">someBlacklist</span><span class="token punctuation">;</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>Denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">SOME_BLACKLIST</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">payload</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">blacklist</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token keyword">const</span> <span class="token variable">payload</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">blacklist</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">}</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">payload</span><span class="token punctuation">.</span><span class="token variable">blacklist</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">payload</span><span class="token punctuation">.</span><span class="token variable">blacklist</span> <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">payload</span><span class="token punctuation">.</span><span class="token variable">blacklist</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">payload</span><span class="token punctuation">.</span><span class="token variable">blacklist</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
