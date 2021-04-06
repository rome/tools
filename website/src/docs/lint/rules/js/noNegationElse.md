---
title: Lint Rule js/noNegationElse
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eslint-rule: https://eslint.org/docs/rules/no-negated-condition
eleventyNavigation:
	key: lint-rules/js/noNegationElse
	parent: lint-rules
	title: js/noNegationElse
---

# js/noNegationElse

<!-- GENERATED:START(hash:b0bbf8aa76294879c8b7e9aea8dce916bf76392e,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION

**ESLint Equivalent:** [no-negated-condition](https://eslint.org/docs/rules/no-negated-condition)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:6a389db02dba8581067186d27fdedee10fb577c7,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token boolean">true</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token variable">consequent</span><span class="token punctuation">;</span><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token variable">alternate</span><span class="token punctuation">;</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noNegationElse</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Invert blocks</strong></span><span style="color: Tomato;"> when performing a negation test.</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token boolean">true</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token variable">consequent</span><span class="token punctuation">;</span><span class="token punctuation">}</span> <span class="token keyword">else</span> <span class="token punctuation">{</span><span class="token variable">alternate</span><span class="token punctuation">;</span><span class="token punctuation">}</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">if</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">true)</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">{</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: Tomato;"><strong>consequ</strong></span><span style="color: Tomato;">ent;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">if</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">(true)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">{</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: MediumSeaGreen;"><strong>alt</strong></span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;">;</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong>3</strong><strong> │ </strong>  }<span style="opacity: 0.8;">&middot;</span>else<span style="opacity: 0.8;">&middot;</span>{
  <strong>  </strong><strong>4</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: Tomato;"><strong>alt</strong></span><span style="color: Tomato;">e</span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;">n</span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;">t</span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;">;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>4</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&rarr; </span></span><span style="color: MediumSeaGreen;"><strong>consequ</strong></span><span style="color: MediumSeaGreen;">ent;</span>
  <strong>  </strong><strong>5</strong><strong> </strong><strong>5</strong><strong> │ </strong>  }

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js"><span class="token operator">!</span><span class="token boolean">true</span> <span class="token punctuation">?</span> <span class="token variable">consequent</span> <span class="token punctuation">:</span> <span class="token variable">alternate</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/noNegationElse</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;"><strong>Invert blocks</strong></span><span style="color: Tomato;"> when performing a negation test.</span>

    <span class="token operator">!</span><span class="token boolean">true</span> <span class="token punctuation">?</span> <span class="token variable">consequent</span> <span class="token punctuation">:</span> <span class="token variable">alternate</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">Safe fix</span>

  <span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">true</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">?</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">consequent</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>:</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>alternate</strong></span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">true</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">?</span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>alternate</strong></span><span style="color: MediumSeaGreen;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: MediumSeaGreen;"><strong>:</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">consequent</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js"><span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span><span class="token boolean">true</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token variable">consequent</span><span class="token punctuation">;</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js"><span class="token boolean">true</span> <span class="token punctuation">?</span> <span class="token variable">consequent</span> <span class="token punctuation">:</span> <span class="token variable">alternate</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
