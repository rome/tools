---
title: Lint Rule jsx/usePascalCase
layout: layouts/rule.liquid
description: enforce PascalCase for user-defined JSX components
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-pascal-case.md
eleventyNavigation:
	key: lint-rules/jsx/usePascalCase
	parent: lint-rules
	title: jsx/usePascalCase
---

# jsx/usePascalCase

<!-- GENERATED:START(hash:996e1cea854c1870d7073415ad81c36aef9e6b5f,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce PascalCase for user-defined JSX components

**ESLint Equivalent:** [jsx-pascal-case](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-pascal-case.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:589bceddc79b24a08e3dcbb5386491e197fffeb7,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples

### Invalid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">Foo_component</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx/usePascalCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Switch </span><span style="color: Tomato;"><strong>Foo_component</strong></span><span style="color: Tomato;"> to </span><span style="color: Tomato;"><strong>FooComponent</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token variable">Foo_component</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">User-defined JSX components should be defined and referenced in</span>
    <span style="color: rgb(38, 148, 255);">PascalCase.</span>

</code></pre>{% endraw %}

---

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">FOO_COMPONENT</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx/usePascalCase</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Switch </span><span style="color: Tomato;"><strong>FOO_COMPONENT</strong></span><span style="color: Tomato;"> to </span><span style="color: Tomato;"><strong>FOOCOMPONENT</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token variable">FOO_COMPONENT</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: rgb(38, 148, 255);">ℹ </span></strong><span style="color: rgb(38, 148, 255);">User-defined JSX components should be defined and referenced in</span>
    <span style="color: rgb(38, 148, 255);">PascalCase.</span>

</code></pre>{% endraw %}

### Valid

{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">Foo</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">foo_bar</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">fooBar</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">foo_COMPONENT</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">foo</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">div</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-js"><code class="language-js">&lt;<span class="token variable">FooComponent</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
