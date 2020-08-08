---
title: Lint Rule js/emptyMatches
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/emptyMatches
	parent: lint-rules
	title: js/emptyMatches
---

# js/emptyMatches

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:8e2efcc194e36786fb2eb7ccee4e32b17ff079e7,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token regex">/a*/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:8</span> <strong>lint/js/emptyMatches</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This expression can return </span><span style="color: Tomato;"><strong>empty matches</strong></span><span style="color: Tomato;">, and may match infinitely in</span>
    <span style="color: Tomato;">some use cases.</span>

    <span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token regex">/a*/</span>
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Strengthen the regular expression so that empty matches are not</span>
    <span style="color: DodgerBlue;">possible.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">b</span> <span class="token operator">=</span> <span class="token regex">/a*(abc)?[1,2,3]*/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:8</span> <strong>lint/js/emptyMatches</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This expression can return </span><span style="color: Tomato;"><strong>empty matches</strong></span><span style="color: Tomato;">, and may match infinitely in</span>
    <span style="color: Tomato;">some use cases.</span>

    <span class="token keyword">let</span> <span class="token variable">b</span> <span class="token operator">=</span> <span class="token regex">/a*(abc)?[1,2,3]*/</span>
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Strengthen the regular expression so that empty matches are not</span>
    <span style="color: DodgerBlue;">possible.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token regex">/a*(abc)+[1,2,3]?/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">b</span> <span class="token operator">=</span> <span class="token regex">/a+(abc)*/</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
