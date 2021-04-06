---
title: Lint Rule js/noTemplateCurlyInString
layout: layouts/rule.liquid
description: disallow template literal placeholder syntax in regular strings
eslint-rule: https://eslint.org/docs/rules/no-template-curly-in-string
eleventyNavigation:
	key: lint-rules/js/noTemplateCurlyInString
	parent: lint-rules
	title: js/noTemplateCurlyInString
---

# js/noTemplateCurlyInString

<!-- GENERATED:START(hash:ed6b8de33637bb30134c048a9f3ad09ddcbd687a,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow template literal placeholder syntax in regular strings

**ESLint Equivalent:** [no-template-curly-in-string](https://eslint.org/docs/rules/no-template-curly-in-string)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:64bda8ccb20e9bb3470520d84637381e43307cd2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">const</span> <span class="token variable">user</span> <span class="token operator">=</span> <span class="token string">&apos;Faustina&apos;</span><span class="token punctuation">;</span>
               <span class="token keyword">const</span> <span class="token variable">helloUser</span> <span class="token operator">=</span> <span class="token string">&apos;Hello, ${user}&apos;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:2:33</span> <strong>lint/js/noTemplateCurlyInString</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This string contains an </span><span style="color: Tomato;"><strong>unexpected template string</strong></span><span style="color: Tomato;"> expression.</span>

  <strong>  1</strong><strong> │ </strong><span class="token keyword">const</span> <span class="token variable">user</span> <span class="token operator">=</span> <span class="token string">&apos;Faustina&apos;</span><span class="token punctuation">;</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>               <span class="token keyword">const</span> <span class="token variable">helloUser</span> <span class="token operator">=</span> <span class="token string">&apos;Hello, ${user}&apos;</span><span class="token punctuation">;</span>
     <strong> │ </strong>                                 <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Using template string expressions in regular strings is usually a</span>
    <span style="color: rgb(38, 148, 255);">typo.</span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
