---
title: Lint Rule js/noTemplateCurlyInString
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noTemplateCurlyInString
	parent: lint-rules
	title: js/noTemplateCurlyInString
---

# js/noTemplateCurlyInString

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:5ac6065a5ca2efc62130ed003a22f274a24cf708,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">user</span> <span class="token operator">=</span> <span class="token string">&apos;Faustina&apos;</span><span class="token punctuation">;</span>
               <span class="token keyword">const</span> <span class="token variable">helloUser</span> <span class="token operator">=</span> <span class="token string">&apos;Hello, ${user};</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:49</span> <strong>parse/js</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Unterminated string constant</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">user</span> <span class="token operator">=</span> <span class="token string">&apos;Faustina&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>               <span class="token keyword">const</span> <span class="token variable">helloUser</span> <span class="token operator">=</span> <span class="token string">&apos;Hello, ${user};</span>
     <strong> │ </strong>                                                 <span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
