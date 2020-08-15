---
title: Lint Rule regex/noEmptyMatches
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/regex/noEmptyMatches
	parent: lint-rules
	title: regex/noEmptyMatches
---

# regex/noEmptyMatches

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:483471e289675d907d908d57b5930402fd0dba48,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">a</span> <span class="token operator">=</span> <span class="token regex">/a*/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:8</span> <strong>lint/regex/noEmptyMatches</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:8</span> <strong>lint/regex/noEmptyMatches</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
<!-- GENERATED:END(id:examples) -->
