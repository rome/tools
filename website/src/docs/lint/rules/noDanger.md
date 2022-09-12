---
title: Lint Rule noDanger
layout: layouts/rule.liquid
---

# noDanger (since v0.10.0)

> This rule is recommended by Rome.

Prevent the usage of dangerous JSX props

## Examples

### Invalid

```jsx
<div dangerouslySetInnerHTML={{ __html: 'child' }}></div>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noDanger/">react/noDanger</a></span><span style="color: Tomato;">]</span><em>: </em><em>Avoid passing content using the </em><em><em>dangerouslySetInnerHTML</em></em><em> prop.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> react/noDanger.js:1:6
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;div <span style="color: Tomato;">d</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">l</span><span style="color: Tomato;">y</span><span style="color: Tomato;">S</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">H</span><span style="color: Tomato;">T</span><span style="color: Tomato;">M</span><span style="color: Tomato;">L</span>={{ __html: 'child' }}&gt;&lt;/div&gt;
  <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

= <span style="color: Orange;">warning</span><span style="color: Orange;">: </span>Setting content using code can expose users to cross-site scripting (XSS) attacks

</code></pre>{% endraw %}

```jsx
React.createElement('div', {
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;">SyntaxError</span><span style="color: Tomato;">]</span><em>: </em><em>expected `}` but instead the file ends</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> react/noDanger.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> 
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span> <span style="color: Tomato;">the file ends here</span>

</code></pre>{% endraw %}

