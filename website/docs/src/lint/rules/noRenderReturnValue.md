---
title: Lint Rule noRenderReturnValue
layout: layouts/docs.liquid
---

# noRenderReturnValue (since v0.10.0)

> This rule is recommended by Rome.

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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noRenderReturnValue.js:1:13 <a href="https://docs.rome.tools/lint/rules/noRenderReturnValue">lint/correctness/noRenderReturnValue</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not depend on the value returned by the function </span><span style="color: Tomato;"><strong>ReactDOM.render()</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const foo = ReactDOM.render(&lt;div /&gt;, document.body);
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The returned value is legacy and future versions of react might return that value asynchronously.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://facebook.github.io/react/docs/react-dom.html#render">React documentation</a></span><span style="color: rgb(38, 148, 255);"> for more information.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
ReactDOM.render(<div />, document.body);
```

