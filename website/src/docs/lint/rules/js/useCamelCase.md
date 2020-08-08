---
title: Lint Rule js/useCamelCase
layout: layouts/rule.liquid
description: enforce camelcase naming convention
layout-type: single
eslint-rule: https://eslint.org/docs/rules/camelcase
eleventyNavigation:
	key: lint-rules/js/useCamelCase
	parent: lint-rules
	title: js/useCamelCase
---

# js/useCamelCase

<!-- GENERATED:START(hash:4b19cadf65179cce1b1ca80d34bf2ad434f23acd,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce camelcase naming convention

**ESLint Equivalent:** [camelcase](https://eslint.org/docs/rules/camelcase)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:e56cc5d7029b96cc7d1357cacdc2d3e38d66d988,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">underscore_case</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">_underscore_case</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token variable">obj</span><span class="token punctuation">.</span><span class="token variable">underscore_case</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
