---
title: Lint Rule regex/noDuplicateGroupNamesInRegularExpressions
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/regex/noDuplicateGroupNamesInRegularExpressions
	parent: lint-rules
	title: regex/noDuplicateGroupNamesInRegularExpressions
---

# regex/noDuplicateGroupNamesInRegularExpressions

<!-- GENERATED:START(hash:0c842939a2a049e8cdb4f0be47baab529ffcaf37,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:349eb6ca05c78b09b54b49896fccf2f8c04a9a25,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-ts"><code class="language-ts"><span class="token regex">/(?&lt;month&gt;[0-9])-(?&lt;year&gt;[0-9])-(?&lt;month&gt;[0-9])-(?&lt;year&gt;[0-9])-(?&lt;day&gt;[0-9])-([0-9])-(?&lt;month&gt;[0-9])/</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.ts:1:32</span> <strong>parse(regex)</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Duplicate capture group name</span>

    /(?&lt;month&gt;[0-9])-(?&lt;year&gt;[0-9])-(?&lt;month&gt;[0-9])-(?&lt;year&gt;[0-9])-(?&lt;day
      &gt;[0-9])-([0-9])-(?&lt;month&gt;[0-9])/
    <span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
