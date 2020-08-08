---
title: Lint Rule js/noConfusingLanguage
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noConfusingLanguage
	parent: lint-rules
	title: js/noConfusingLanguage
---

# js/confusingLanguage

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:9075c1ba3f7a59211cac3a922e5cb367631f848f,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">//	the	blacklist</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:5</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

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
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">/*</span><span class="token comment">  </span><span class="token comment">the</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token comment">blacklist</span><span class="token comment">  </span><span class="token comment">*/</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">blacklist</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">blacklist</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">BLACKLIST</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">BLACKLIST</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>DENYLIST</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">someBlacklist</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:4</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">someBlacklist</span><span class="token punctuation">;</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>Denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">SOME_BLACKLIST</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:5</span> <strong>lint/js/noConfusingLanguage</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">SOME_BLACKLIST</span><span class="token punctuation">;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>DENYLIST</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">payload</span> <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token variable">blacklist</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">}</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noConfusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noConfusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noConfusingLanguage</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">payload</span><span class="token punctuation">.</span><span class="token variable">blacklist</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
