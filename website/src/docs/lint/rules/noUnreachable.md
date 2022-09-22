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

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnreachable.js:3:5 <a href="https://rome.tools/docs/lint/rules/noUnreachable">lint/nursery/noUnreachable</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This code is unreachable</span>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">... before it can reach this code</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:3:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">v</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This statement will return from the function ...</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:2:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;">u</span><span style="color: Tomato;">r</span><span style="color: Tomato;">n</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
function example() {
    for(let i = 0; i < 10; ++i) {
        break;
    }
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnreachable.js:2:28 <a href="https://rome.tools/docs/lint/rules/noUnreachable">lint/nursery/noUnreachable</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This code is unreachable</span>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This code will never be reached ...</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:2:28
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     for(let i = 0; i &lt; 10; <span style="color: Tomato;">+</span><span style="color: Tomato;">+</span><span style="color: Tomato;">i</span>) {
    <span style="color: rgb(38, 148, 255);">│</span>                            <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">... because this statement will break the flow of the code beforehand</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:3:9
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">b</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">k</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
function example() {
    for(const key in value) {
        continue;
        neverCalled();
    }
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnreachable.js:4:9 <a href="https://rome.tools/docs/lint/rules/noUnreachable">lint/nursery/noUnreachable</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This code is unreachable</span>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">... before it can reach this code</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:4:9
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">v</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">C</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This statement will continue the loop ...</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnreachable.js:3:9
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

