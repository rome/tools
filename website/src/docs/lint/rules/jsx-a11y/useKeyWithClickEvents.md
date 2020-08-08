---
title: Lint Rule jsx-a11y/useKeyWithClickEvents
layout: layouts/rule.liquid
description: enforce a clickable non-interactive element has at least one keyboard event listener.
eslint-rule: https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/click-events-have-key-events.md
eleventyNavigation:
	key: lint-rules/jsx-a11y/useKeyWithClickEvents
	parent: lint-rules
	title: jsx-a11y/useKeyWithClickEvents
---

# jsx-a11y/useKeyWithClickEvents

<!-- GENERATED:START(hash:205609b44502cd0b3106ebb9358ab3a06c613eb3,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
enforce a clickable non-interactive element has at least one keyboard event listener.

**ESLint Equivalent:** [click-events-have-key-events](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/master/docs/rules/click-events-have-key-events.md)
<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:7676f72d0ef8a5041b089d276b54d621eedcf274,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1</span> <strong>lint/jsx-a11y/useKeyWithClickEvents</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Pair the </span><span style="color: Tomato;"><strong>onClick</strong></span><span style="color: Tomato;"> mouse event with the </span><span style="color: Tomato;"><strong>onKeyUp</strong></span><span style="color: Tomato;">, the </span><span style="color: Tomato;"><strong>onKeyDown</strong></span><span style="color: Tomato;">, or the </span>
    <span style="color: Tomato;"><strong>onKeyPress</strong></span><span style="color: Tomato;"> keyboard event.</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Actions triggered using mouse events should have corresponding</span>
    <span style="color: DodgerBlue;">keyboard events to account for keyboard-only navigation.</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token attr-name">onKeyDown</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">handleKeyDown</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token attr-name">onKeyUp</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">handleKeyUp</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">onClick</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token operator">=&gt;</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">}</span> <span class="token attr-name">onKeyPress</span><span class="token operator">=</span><span class="token punctuation">{</span><span class="token keyword">this</span><span class="token punctuation">.</span><span class="token variable">handleKeyPress</span><span class="token punctuation">}</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
