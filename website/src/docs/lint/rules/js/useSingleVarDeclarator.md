<<<<<<< HEAD:website/src/docs/lint/rules/js/singleVarDeclarator.md
---
title: Lint Rule js/singleVarDeclarator
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/singleVarDeclarator
	parent: lint-rules
	title: js/singleVarDeclarator
---

# js/singleVarDeclarator

=======
---
title: Lint Rule js/useSingleVarDeclarator
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useSingleVarDeclarator
	parent: lint-rules
	title: js/useSingleVarDeclarator
---

# js/useSingleVarDeclarator

>>>>>>> feat: consolidate lint rule naming:website/src/docs/lint/rules/js/useSingleVarDeclarator.md
MISSING DOCUMENTATION

<!-- GENERATED:START(hash:05e2ea1fc5bb387f5f88f278c4ffe442ec6dd30f,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">bar</span><span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1</span> <strong>lint/js/useSingleVarDeclarator</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Declare variables separately.</span>

    <span class="token keyword">let</span> <span class="token variable">foo</span><span class="token punctuation">,</span> <span class="token variable">bar</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">let</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">foo</span><span style="color: Tomato;"><strong>,</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">bar;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">let</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">foo</span><span style="color: MediumSeaGreen;"><strong>;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>let</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">bar;</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token function">let</span> <span class="token function">i</span> <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">,</span> <span class="token function">x</span> <span class="token operator">=</span> <span class="token number">1</span><span class="token punctuation">;</span> <span class="token function">i</span> <span class="token operator">&lt;</span> <span class="token function">arr</span><span class="token punctuation">.</span><span class="token function">length</span><span class="token punctuation">;</span> <span class="token function">i</span><span class="token operator">++</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
