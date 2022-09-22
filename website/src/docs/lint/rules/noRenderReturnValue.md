---
title: Lint Rule noRenderReturnValue
layout: layouts/rule.liquid
---

# noRenderReturnValue (since v0.10.0)

Prevent the usage of the return value of `React.render`.

>`ReactDOM.render()` currently returns a reference to the root `ReactComponent` instance. However, using this return value is legacy
and should be avoided because future versions of React may render components asynchronously in some cases.
If you need a reference to the root `ReactComponent` instance, the preferred solution is to attach a [callback ref](https://reactjs.org/docs/refs-and-the-dom.html#callback-refs)
to the root element.


Source: [ReactDOM documentation](https://facebook.github.io/react/docs/react-dom.html#render)

## Examples

### Invalid

```jsx
const foo = ReactDOM.render(<div />, document.body);
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noRenderReturnValue.js:1:13 <a href="https://rome.tools/docs/lint/rules/noRenderReturnValue">lint/nursery/noRenderReturnValue</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Do not depend on the value returned by the function </span><span style="color: Orange;"><strong>ReactDOM.render()</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noRenderReturnValue.js:1:13
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = <span style="color: Tomato;">R</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">D</span><span style="color: Tomato;">O</span><span style="color: Tomato;">M</span><span style="color: Tomato;">.</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">(</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;"> </span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">d</span><span style="color: Tomato;">o</span><span style="color: Tomato;">c</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">.</span><span style="color: Tomato;">b</span><span style="color: Tomato;">o</span><span style="color: Tomato;">d</span><span style="color: Tomato;">y</span><span style="color: Tomato;">)</span>;
    <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The returned value is legacy and future versions of react might return that value asynchronously.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://facebook.github.io/react/docs/react-dom.html#render">React documentation</a></span><span style="color: rgb(38, 148, 255);"> for more information.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
ReactDOM.render(<div />, document.body);
```

