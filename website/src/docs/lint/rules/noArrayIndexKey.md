---
title: Lint Rule noArrayIndexKey
layout: layouts/rule.liquid
---

# noArrayIndexKey (since v0.10.0)

Discourage the usage of Array index in keys.

>We don’t recommend using indexes for keys if the order of items may change.
This can negatively impact performance and may cause issues with component state.
Check out Robin Pokorny’s article for an
[in-depth explanation on the negative impacts of using an index as a key](https://robinpokorny.com/blog/index-as-a-key-is-an-anti-pattern/).
If you choose not to assign an explicit key to list items then React will default to using indexes as keys.


Source [React documentation](https://reactjs.org/docs/lists-and-keys.html#keys)

## Examples

### Invalid

```jsx
something.forEach((Element, index) => {
    <Component key={index} >foo</Component>
});
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noArrayIndexKey.js:2:21 <a href="https://rome.tools/docs/lint/rules/noArrayIndexKey">lint/nursery/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using the index of an array as key property in an element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:2:21
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     &lt;Component key={<span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>} &gt;foo&lt;/Component&gt;
    <span style="color: rgb(38, 148, 255);">│</span>                     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is the source of the key value.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:1:29
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> something.forEach((Element, <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>) =&gt; {
    <span style="color: rgb(38, 148, 255);">│</span>                             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The order of the items may change, and this also affects performances and component state.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">React documentation</a></span><span style="color: rgb(38, 148, 255);">. </span>
  
</code></pre>{% endraw %}

```jsx
React.Children.map(this.props.children, (child, index) => (
    React.cloneElement(child, { key: index })
))
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noArrayIndexKey.js:2:38 <a href="https://rome.tools/docs/lint/rules/noArrayIndexKey">lint/nursery/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using the index of an array as key property in an element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:2:38
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     React.cloneElement(child, { key: <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span> })
    <span style="color: rgb(38, 148, 255);">│</span>                                      <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is the source of the key value.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noArrayIndexKey.js:1:49
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> React.Children.map(this.props.children, (child, <span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span>) =&gt; (
    <span style="color: rgb(38, 148, 255);">│</span>                                                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The order of the items may change, and this also affects performances and component state.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">React documentation</a></span><span style="color: rgb(38, 148, 255);">. </span>
  
</code></pre>{% endraw %}

