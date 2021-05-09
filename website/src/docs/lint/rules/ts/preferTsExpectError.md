---
title: Lint Rule ts/preferTsExpectError
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/ts/preferTsExpectError
	parent: lint-rules
	title: ts/preferTsExpectError
---

# ts/preferTsExpectError

Prefer `@ts-expect-error` to get notified when suppression is no longer necessary.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:72776c06eb93739ef334af9f392b50f898fbb8b7,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-ignore</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">//		@ts-ignore</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/* @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/** @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> * @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

---

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> ** @ts-ignore */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"></code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">// @ts-expect-error</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">//		@ts-expect-error</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/* @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/** @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> * @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token comment">/**</span>
<span class="token comment"> ** @ts-expect-error */</span>
<span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">:</span> <span class="token variable">boolean</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
