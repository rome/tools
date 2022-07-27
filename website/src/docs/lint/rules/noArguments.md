---
title: Lint Rule noArguments
layout: layouts/rule.liquid
---

# noArguments (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of `arguments`

## Examples

### Invalid

```jsx
function f() {
   console.log(arguments);
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noArguments/">js/noArguments</a></span><span style="color: Orange;">]</span><em>: </em><em>Use the </em><em><em>rest parameters</em></em><em> instead of </em><em><em>arguments</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noArguments.js:2:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>    console.log(arguments);
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: <em>arguments</em> does not have <em>Array.prototype</em> methods and can be inconvenient to use.

</code></pre>{% endraw %}

### Valid

```js
function f() {
    let arguments = 1;
    console.log(arguments);
}
```

