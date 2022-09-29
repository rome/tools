---
title: Lint Rule noArrayIndexKey
layout: layouts/rule.liquid
---

# noArrayIndexKey (since v0.10.0)

Prevent the usage of Array index in keys

## Examples

### Invalid

```jsx
something.forEach((Element, index) => {
    <Component key={index} >foo</Component>
});
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noArrayIndexKey.js:2:21 <a href="https://rome.tools/docs/lint/rules/noArrayIndexKey">lint/nursery/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using array index as key property in an element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:2:21
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     &lt;Component key={<span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>} &gt;foo&lt;/Component&gt;
    <span style="color: rgb(38, 148, 255);">│</span>                     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the key comes from.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:1:29
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> something.forEach((Element, <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>) =&gt; {
    <span style="color: rgb(38, 148, 255);">│</span>                             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
React.Children.map(this.props.children, (child, index) => (
    React.cloneElement(child, { key: index })
))
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noArrayIndexKey.js:2:38 <a href="https://rome.tools/docs/lint/rules/noArrayIndexKey">lint/nursery/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using array index as key property in an element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:2:38
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     React.cloneElement(child, { key: <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span> })
    <span style="color: rgb(38, 148, 255);">│</span>                                      <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the key comes from.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:1:49
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> React.Children.map(this.props.children, (child, <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>) =&gt; (
    <span style="color: rgb(38, 148, 255);">│</span>                                                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

