<<<<<<< HEAD:website/src/docs/lint/rules/js/sparseArray.md
---
title: Lint Rule js/sparseArray
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/sparseArray
	parent: lint-rules
	title: js/sparseArray
---

# js/sparseArray

=======
---
title: Lint Rule js/noSparseArray
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/noSparseArray
	parent: lint-rules
	title: js/noSparseArray
---

# js/noSparseArray

>>>>>>> feat: consolidate lint rule naming:website/src/docs/lint/rules/js/noSparseArray.md
MISSING DOCUMENTATION

<!-- GENERATED:START(hash:de96ae471bbc32ba9470fe071c43c3a3bdce3c5f,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:3</span> <strong>lint/js/noSparseArray</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>array</strong></span><span style="color: Tomato;"> contains an </span><span style="color: Tomato;"><strong>empty slot</strong></span><span style="color: Tomato;">.</span>

    <span class="token punctuation">[</span><span class="token number">1</span><span class="token punctuation">,</span><span class="token punctuation">,</span><span class="token number">2</span><span class="token punctuation">]</span>
       <span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Sparse arrays without values for some items can lead to confusion.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <span style="color: Tomato;">-</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>undefined</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->
