---
title: Lint Rule ts/useInterfaces
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eslint-rule: https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-filename-extension.md
eleventyNavigation:
	key: lint-rules/ts/useInterfaces
	parent: lint-rules
	title: ts/useInterfaces
---

# ts/useInterfaces

<!-- GENERATED:START(hash:d73068d2ce9c2c77289bff157d4d7371ad605383,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
MISSING DOCUMENTATION

**ESLint Equivalent:** [jsx-filename-extension](https://github.com/yannickcr/eslint-plugin-react/blob/master/docs/rules/jsx-filename-extension.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:5261e89f4402a9ef8bfae90de8acad36b8dc7a72,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">div</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">test.js:2</span> <strong>lint/jsx/fileExtension</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Files with the </span><span style="color: Tomato;"><strong>.js</strong></span><span style="color: Tomato;"> extension cannot contain JSX elements.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// @jsx</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>&lt;<span class="token attr-name">div</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Change the </span><span style="color: DodgerBlue;"><strong>test.js</strong></span><span style="color: DodgerBlue;"> file extension to </span><span style="color: DodgerBlue;"><strong>.jsx</strong></span><span style="color: DodgerBlue;"> or </span><span style="color: DodgerBlue;"><strong>.tsx</strong></span><span style="color: DodgerBlue;">.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;&gt;&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">test.js:2</span> <strong>lint/jsx/fileExtension</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Files with the </span><span style="color: Tomato;"><strong>.js</strong></span><span style="color: Tomato;"> extension cannot contain JSX elements.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// @jsx</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>&lt;&gt;&lt;<span class="token operator">/</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Change the </span><span style="color: DodgerBlue;"><strong>test.js</strong></span><span style="color: DodgerBlue;"> file extension to </span><span style="color: DodgerBlue;"><strong>.jsx</strong></span><span style="color: DodgerBlue;"> or </span><span style="color: DodgerBlue;"><strong>.tsx</strong></span><span style="color: DodgerBlue;">.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">test.js:2</span> <strong>lint/jsx/fileExtension</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Files with the </span><span style="color: Tomato;"><strong>.js</strong></span><span style="color: Tomato;"> extension cannot contain JSX elements.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// @jsx</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>&lt;<span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Change the </span><span style="color: DodgerBlue;"><strong>test.js</strong></span><span style="color: DodgerBlue;"> file extension to </span><span style="color: DodgerBlue;"><strong>.jsx</strong></span><span style="color: DodgerBlue;"> or </span><span style="color: DodgerBlue;"><strong>.tsx</strong></span><span style="color: DodgerBlue;">.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">test.js:2</span> <strong>lint/jsx/fileExtension</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Files with the </span><span style="color: Tomato;"><strong>.js</strong></span><span style="color: Tomato;"> extension cannot contain JSX elements.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">// @jsx</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong>&lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Change the </span><span style="color: DodgerBlue;"><strong>test.js</strong></span><span style="color: DodgerBlue;"> file extension to </span><span style="color: DodgerBlue;"><strong>.jsx</strong></span><span style="color: DodgerBlue;"> or </span><span style="color: DodgerBlue;"><strong>.tsx</strong></span><span style="color: DodgerBlue;">.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token string">&apos;&lt;div&gt;&lt;/div&gt;&apos;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">div</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;&gt;&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">div</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">div</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;&gt;&lt;<span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token comment">// @jsx</span>
&lt;<span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;&lt;<span class="token operator">/</span><span class="token attr-name">React</span><span class="token punctuation">.</span><span class="token attr-name">Fragment</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
