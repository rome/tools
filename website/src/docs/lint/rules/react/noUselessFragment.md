---
title: Lint Rule react/noUselessFragment
layout: layouts/rule.liquid
description: disallow unnecessary fragments
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-useless-fragment.md
eleventyNavigation:
	key: lint-rules/react/noUselessFragment
	parent: lint-rules
	title: react/noUselessFragment
---

# react/noUselessFragment

<!-- GENERATED:START(hash:b6e099bff0712ef912ed4d2898880643f57b9614,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
disallow unnecessary fragments

**ESLint Equivalent:** [jsx-no-useless-fragment](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-no-useless-fragment.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:00664948b76338c6edebc05d8581497b89c62485,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;&gt;<span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

    &lt;&gt;<span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">{foo}</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/&gt;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">{foo}</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">p</span>&gt;&lt;&gt;foo&lt;<span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">p</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:3</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token attr-name">p</span>&gt;&lt;&gt;foo&lt;<span class="token operator">/</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">p</span>&gt;
       <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">foo</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/&gt;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;&gt;&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

    &lt;&gt;&lt;<span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/&gt;</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;React.Fragment&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">foo</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/React.Fragment&gt;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">Fragment</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

    &lt;<span class="token attr-name">Fragment</span>&gt;foo&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;Fragment&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">foo</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/Fragment&gt;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">foo</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">section</span>&gt;
					&lt;&gt;
						&lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
						&lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
					&lt;<span class="token operator">/</span>&gt;
				&lt;<span class="token operator">/</span><span class="token attr-name">section</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:2:5</span> <strong>lint/react/noUselessFragment</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid using unnecessary </span><span style="color: Tomato;"><strong>Fragment</strong></span><span style="color: Tomato;">.</span>

  <strong>  1</strong><strong> │ </strong>&lt;<span class="token attr-name">section</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>          &lt;&gt;
     <strong> │ </strong>          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 3</strong><strong> │ </strong>            &lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 4</strong><strong> │ </strong>            &lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 5</strong><strong> │ </strong>          &lt;<span class="token operator">/</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>
  <strong>  6</strong><strong> │ </strong>        &lt;<span class="token operator">/</span><span class="token attr-name">section</span>&gt;

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">A </span><span style="color: DodgerBlue;"><strong>Fragment</strong></span><span style="color: DodgerBlue;"> is redundant if it contains only one child, or if it is</span>
    <span style="color: DodgerBlue;">the child of a html element, and is not a keyed fragment.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

  <strong>  </strong><strong>1</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;&gt;</strong></span>
  <strong>  </strong><strong>2</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">&lt;div</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">/&gt;</span>
  <strong>  </strong><strong>3</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong><span style="opacity: 0.8;">&rarr; </span></strong></span><span style="color: Tomato;">&lt;div</span><span style="color: Tomato;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: Tomato;">/&gt;</span>
  <strong>  </strong><strong>4</strong><strong> </strong><strong> </strong><strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;/&gt;</strong></span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;div</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>
  <strong>  </strong><strong> </strong><strong> </strong><strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;div</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">&middot;</span></span><span style="color: MediumSeaGreen;">/&gt;</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;&gt;
	&lt;<span class="token attr-name">Foo</span> <span class="token operator">/</span>&gt;
	&lt;<span class="token attr-name">Bar</span> <span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;&gt;foo <span class="token punctuation">{</span><span class="token variable">bar</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;&gt; <span class="token punctuation">{</span><span class="token variable">foo</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">cat</span> <span class="token operator">=</span> &lt;&gt;meow&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">const</span> <span class="token variable">cat</span> <span class="token operator">=</span> <span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> &lt;&gt;meow&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">cat</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">return</span> &lt;&gt;meow&lt;<span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">function</span> <span class="token variable">cat</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span>
	<span class="token keyword">const</span> <span class="token variable">foo</span> <span class="token operator">=</span> &lt;&gt;meow&lt;<span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
	<span class="token keyword">return</span> <span class="token variable">foo</span><span class="token punctuation">;</span>
<span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">SomeComponent</span>&gt;
	&lt;&gt;
		&lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
		&lt;<span class="token attr-name">div</span> <span class="token operator">/</span>&gt;
	&lt;<span class="token operator">/</span>&gt;
&lt;<span class="token operator">/</span><span class="token attr-name">SomeComponent</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">Fragment</span> <span class="token attr-name">key</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">item</span><span class="token punctuation">.</span><span class="token variable">id</span><span class="token punctuation">}</span>&gt;<span class="token punctuation">{</span><span class="token variable">item</span><span class="token punctuation">.</span><span class="token variable">value</span><span class="token punctuation">}</span>&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
