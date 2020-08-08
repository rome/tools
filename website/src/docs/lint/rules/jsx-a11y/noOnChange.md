---
title: Lint Rule jsx-a11y/noOnChange
layout: layouts/rule.liquid
description: discourage the usage of `onChange`
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-onchange.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/noOnChange
	parent: lint-rules
	title: jsx-a11y/noOnChange
---

# jsx-a11y/noOnChange

It discourages the usage of the event handler `onChange` on input fields and it promotes the usage
of `onBlur`, which is more reliable using the keyboard.


## Accessibility guidelines

- [WCAG 3.2.2](https://www.w3.org/WAI/WCAG21/Understanding/on-input)

### Resources

- [onChange Event Accessibility Issues](http://cita.disability.uiuc.edu/html-best-practices/auto/onchange.php)
- [onChange Select Menu](http://www.themaninblue.com/writing/perspective/2004/10/19/)

<!-- GENERATED:START(hash:ee36f9cc2626b9aa23f446c3baf6834c10c0120a,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
**ESLint Equivalent:** [no-onchange](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/no-onchange.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:765b474011f8c5556396840f7104be4cc91bf82b,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">select</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noOnChange</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide an </span><span style="color: Tomato;"><strong>onBlur</strong></span><span style="color: Tomato;"> event instead of an </span><span style="color: Tomato;"><strong>onChange</strong></span><span style="color: Tomato;"> event unless</span>
    <span style="color: Tomato;">absolutely necessary.</span>

    &lt;<span class="token attr-name">select</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The onBlur event is more declarative and reliable for indicating</span>
    <span style="color: DodgerBlue;">input changes when using keyboard navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">select</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">handleOnChange</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noOnChange</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide an </span><span style="color: Tomato;"><strong>onBlur</strong></span><span style="color: Tomato;"> event instead of an </span><span style="color: Tomato;"><strong>onChange</strong></span><span style="color: Tomato;"> event unless</span>
    <span style="color: Tomato;">absolutely necessary.</span>

    &lt;<span class="token attr-name">select</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">handleOnChange</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span>
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The onBlur event is more declarative and reliable for indicating</span>
    <span style="color: DodgerBlue;">input changes when using keyboard navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noOnChange</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide an </span><span style="color: Tomato;"><strong>onBlur</strong></span><span style="color: Tomato;"> event instead of an </span><span style="color: Tomato;"><strong>onChange</strong></span><span style="color: Tomato;"> event unless</span>
    <span style="color: Tomato;">absolutely necessary.</span>

    &lt;<span class="token attr-name">option</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The onBlur event is more declarative and reliable for indicating</span>
    <span style="color: DodgerBlue;">input changes when using keyboard navigation.</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:8</span> <strong>lint/jsx-a11y/noOnChange</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Provide an </span><span style="color: Tomato;"><strong>onBlur</strong></span><span style="color: Tomato;"> event instead of an </span><span style="color: Tomato;"><strong>onChange</strong></span><span style="color: Tomato;"> event unless</span>
    <span style="color: Tomato;">absolutely necessary.</span>

    &lt;<span class="token attr-name">option</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The onBlur event is more declarative and reliable for indicating</span>
    <span style="color: DodgerBlue;">input changes when using keyboard navigation.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">select</span> <span class="token attr-name">onBlur</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">select</span> <span class="token attr-name">onBlur</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">handleOnBlur</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token attr-name">onBlur</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">option</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token variable">handleOnChange</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token attr-name">onChange</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;<span class="token punctuation">;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">input</span> <span class="token punctuation">{</span><span class="token operator">...</span><span class="token variable">props</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
