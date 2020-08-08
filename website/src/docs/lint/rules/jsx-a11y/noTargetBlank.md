---
title: Lint Rule jsx-a11y/noTargetBlank
layout: layouts/rule.liquid
description: Prevent usage of unsafe `target="_blank"`
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-target-blank.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noTargetBlank
	parent: lint-rules
	title: jsx-a11y/noTargetBlank
---

# jsx-a11y/noTargetBlank

<!-- GENERATED:START(hash:bd6e02d4cb53d6ceeeb71257890fdd57e86eef05,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
Prevent usage of unsafe `target="_blank"`

**ESLint Equivalent:** [jsx-no-target-blank](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-target-blank.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:17699ba55ae067916c4c6ca55f12c2f7fb87ee54,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;http://external.link&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:31</span> <strong>lint/jsx-a11y/noTargetBlank</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;"> without </span><span style="color: Tomato;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;http://external.link&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;
                                   <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Opening external links in new tabs without rel=&quot;noreferrer&quot; is a</span>
    <span style="color: DodgerBlue;">security risk. See </span>
    <span style="color: DodgerBlue;"><a href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">https://html.spec.whatwg.org/multipage/links.html#link-type-noopener</a></span>
    <span style="color: DodgerBlue;">for more details.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;a</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">href=&quot;http://external.link&quot;</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">href=&quot;http://external.link&quot;&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>child

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">dynamicLink</span><span class="token punctuation">}</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:22</span> <strong>lint/jsx-a11y/noTargetBlank</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;"> without </span><span style="color: Tomato;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">dynamicLink</span><span class="token punctuation">}</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;
                          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Opening external links in new tabs without rel=&quot;noreferrer&quot; is a</span>
    <span style="color: DodgerBlue;">security risk. See </span>
    <span style="color: DodgerBlue;"><a href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">https://html.spec.whatwg.org/multipage/links.html#link-type-noopener</a></span>
    <span style="color: DodgerBlue;">for more details.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;a</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">href={dynamicLink}</span><span style="color: Tomato;"><strong><span style="opacity: 0.8;">&middot;</span></strong></span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;">&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;a</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">href={dynamicLink}&gt;</span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong>2</strong><strong> │ </strong>  <span style="opacity: 0.8;">&rarr; </span>child

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">p</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;http://external.link&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">p</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;http://external.link&apos;</span> <span class="token attr-name">rel</span><span class="token operator">=</span><span class="token string">&apos;noreferrer&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;http://external.link&apos;</span> <span class="token attr-name">rel</span><span class="token operator">=</span><span class="token string">&apos;noopener noreferrer&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;relative/link&apos;</span> <span class="token attr-name">rel</span><span class="token operator">=</span><span class="token string">&apos;noreferrer&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token string">&apos;/absolute/link&apos;</span> <span class="token attr-name">rel</span><span class="token operator">=</span><span class="token string">&apos;noreferrer&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">a</span> <span class="token attr-name">href</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">dynamicLink</span><span class="token punctuation">}</span> <span class="token attr-name">rel</span><span class="token operator">=</span><span class="token string">&apos;noreferrer&apos;</span> <span class="token attr-name">target</span><span class="token operator">=</span><span class="token string">&apos;_blank&apos;</span>&gt;child&lt;<span class="token operator">/</span><span class="token attr-name">a</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
