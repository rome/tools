---
title: Lint Rule noUnreachable
layout: layouts/rule.liquid
---

# noUnreachable (since v0.7.0)

Disallow unreachable code

## Examples

### Invalid

```jsx
function example() {
    return;
    neverCalled();
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnreachable/">nursery/noUnreachable</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>This code is unreachable</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:3:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     return;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">This statement will return from the function ...</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>     neverCalled();
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">... before it can reach this code</span>

</code></pre>{% endraw %}

```jsx
function example() {
    for(let i = 0; i < 10; ++i) {
        break;
    }
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnreachable/">nursery/noUnreachable</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>This code is unreachable</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:2:28
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     for(let i = 0; i &lt; 10; ++i) {
  <span style="color: rgb(38, 148, 255);">│</span>                            <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">This code will never be reached ...</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>         break;
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">... because this statement will break the flow of the code beforehand</span>

</code></pre>{% endraw %}

```jsx
function example() {
    for(const key in value) {
        continue;
        neverCalled();
    }
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnreachable/">nursery/noUnreachable</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>This code is unreachable</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:4:9
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>         continue;
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">This statement will continue the loop ...</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span>         neverCalled();
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">... before it can reach this code</span>

</code></pre>{% endraw %}

