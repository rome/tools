---
title: Lint Rule html/useClosingNonVoid
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
  key: lint-rules/html/useClosingNonVoid
  parent: lint-rules
  title: html/useClosingNonVoid
---

# html/useClosingNonVoid

Close empty HTML elements with an XHTML closing tag.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:ef1b647e1682cc7944a4f430f17fcc0765432fb2,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token attr-name">div</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No known problems!</span>
</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span>child<span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token attr-name">input</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token punctuation">&lt;</span><span class="token attr-name">input</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
