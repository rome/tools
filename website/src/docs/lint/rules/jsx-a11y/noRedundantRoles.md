---
title: Lint Rule jsx-a11y/noRedundantRoles
layout: layouts/rule.liquid
description: enforce explicit role property is not the same as implicit/default role property on element
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-redundant-roles.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noRedundantRoles
	parent: lint-rules
	title: jsx-a11y/noRedundantRoles
---

# jsx-a11y/noRedundantRoles

<!-- GENERATED:START(hash:85c530787f81cc7dae296f80e7e10ff30572f8ac,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce explicit role property is not the same as implicit/default role property on element

**ESLint Equivalent:** [no-redundant-roles](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-redundant-roles.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:1f58a4268d799ab326aa02a5011cb6b5814d3e69,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">article</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:9</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>article</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>article</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">article</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;article&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">article</span>&gt;
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;article</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;article&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;article&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/article&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">button</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;button&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">button</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">button</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;button&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">button</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;button</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;button&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;button&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/button&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;heading&apos;</span> <span class="token attr-name">aria-level</span><span class="token operator">=</span><span class="token string">&apos;1&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;heading&apos;</span> <span class="token attr-name">aria-level</span><span class="token operator">=</span><span class="token string">&apos;1&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute and ARIA attributes.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;h1</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;heading&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>aria-level=&quot;1&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;h1&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/h1&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;heading&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>heading</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>h1</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">h1</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;heading&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">h1</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;h1</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;heading&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;h1&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/h1&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">dialog</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;dialog&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">dialog</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>dialog</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>dialog</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">dialog</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;dialog&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">dialog</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;dialog</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;dialog&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;dialog&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/dialog&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span>  <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:24</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>checkbox</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>input</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">input</span>  <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;checkbox&apos;</span> <span class="token operator">/</span>&gt;
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">type=&quot;checkbox&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>role=&quot;checkbox&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">type=&quot;checkbox&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">figure</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;figure&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">figure</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:9</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>figure</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>figure</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">figure</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;figure&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">figure</span>&gt;
             <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;figure</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;figure&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;figure&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/figure&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">form</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;form&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">form</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>form</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>form</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">form</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;form&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">form</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;form</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;form&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;form&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/form&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">table</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;grid&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">table</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>grid</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>table</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">table</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;grid&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">table</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;table</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;grid&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;table&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/table&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">td</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;gridcell&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">td</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>gridcell</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>td</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">td</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;gridcell&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">td</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;td</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;gridcell&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;td&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/td&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">fieldset</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;group&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">fieldset</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:11</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>group</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>fieldset</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">fieldset</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;group&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">fieldset</span>&gt;
               <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;fieldset</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;group&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;fieldset&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/fieldset&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;img&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:26</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">img</span> <span class="token attr-name">src</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">alt</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span>  <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;img&apos;</span> <span class="token operator">/</span>&gt;
                              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;img</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">src=&quot;foo&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">alt=&quot;bar&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>role=&quot;img&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;img</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">src=&quot;foo&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">alt=&quot;bar&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;link&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:3</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>link</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;link&apos;</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;
       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;a</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;link&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;a&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/a&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">link</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;link&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:6</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>link</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>link</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">link</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;link&apos;</span> <span class="token operator">/</span>&gt;
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;link</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>role=&quot;link&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;link</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">ol</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;list&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">ol</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>list</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>ol</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">ol</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;list&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">ol</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;ol</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;list&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;ol&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/ol&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">ul</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;list&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">ul</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>list</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>ul</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">ul</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;list&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">ul</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;ul</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;list&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;ul&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/ul&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">select</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;listbox&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">select</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>listbox</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>select</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">select</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;listbox&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">select</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;select</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;listbox&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;select&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/select&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">li</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;listitem&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">li</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>listitem</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>li</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">li</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;listitem&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">li</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;li</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;listitem&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;li&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/li&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">nav</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;navigation&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">nav</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>navigation</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>nav</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">nav</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;navigation&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">nav</span>&gt;
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;nav</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;navigation&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;nav&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/nav&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;option&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">option</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>option</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>option</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">option</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;option&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">option</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;option</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;option&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;option&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/option&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">tr</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;row&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tr</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>row</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>tr</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">tr</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;row&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tr</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;tr</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;row&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;tr&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/tr&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">tbody</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tbody</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>rowgroup</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>tbody</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">tbody</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tbody</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;tbody</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;rowgroup&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;tbody&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/tbody&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">tfoot</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tfoot</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>rowgroup</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>tfoot</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">tfoot</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">tfoot</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;tfoot</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;rowgroup&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;tfoot&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/tfoot&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">thead</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">thead</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>rowgroup</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>thead</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">thead</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowgroup&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">thead</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;thead</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;rowgroup&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;thead&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/thead&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">th</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token string">&apos;row&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowheader&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">th</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:16</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>rowheader</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>th</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">th</span> <span class="token attr-name">scope</span><span class="token operator">=</span><span class="token string">&apos;row&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;rowheader&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">th</span>&gt;
                    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;th</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">scope=&quot;row&quot;</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;rowheader&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;th</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">scope=&quot;row&quot;&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/th&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;search&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;searchbox&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:21</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>searchbox</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>input</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;search&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;searchbox&apos;</span> <span class="token operator">/</span>&gt;
                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">type=&quot;search&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>role=&quot;searchbox&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">type=&quot;search&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">table</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;table&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">table</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>table</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>table</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">table</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;table&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">table</span>&gt;
           <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;table</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;table&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;table&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/table&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">dt</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;term&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">dt</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:4</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>term</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>dt</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">dt</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;term&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">dt</span>&gt;
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;dt</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;term&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;dt&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/dt&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">textarea</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;textbox&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">textarea</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:10</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>textbox</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>textarea</strong></span><span style="color: Tomato;"> element is</span>
    <span style="color: Tomato;">redundant.</span>

    &lt;<span class="token attr-name">textarea</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;textbox&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">textarea</span>&gt;
              <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;textarea</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>role=&quot;textbox&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;textarea&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  &lt;/textarea&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;text&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;textbox&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:19</span> <strong>lint/jsx-a11y/noRedundantRoles</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Using the role attribute </span><span style="color: Tomato;"><strong>textbox</strong></span><span style="color: Tomato;"> on the </span><span style="color: Tomato;"><strong>input</strong></span><span style="color: Tomato;"> element is redundant.</span>

    &lt;<span class="token attr-name">input</span> <span class="token attr-name">type</span><span class="token operator">=</span><span class="token string">&apos;text&apos;</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;textbox&apos;</span> <span class="token operator">/</span>&gt;
                       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong>Suggested fix:</strong> Remove the role attribute.

  <span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;input</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">type=&quot;text&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;"><strong>role=&quot;textbox&quot;</strong></span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;">/&gt;</span>
  <span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;input</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">type=&quot;text&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">article</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;presentation&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">article</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">Button</span> <span class="token attr-name">role</span><span class="token operator">=</span><span class="token string">&apos;button&apos;</span> &gt;&lt;<span class="token operator">/</span><span class="token attr-name">Button</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">span</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">span</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
