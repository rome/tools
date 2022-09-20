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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noRenderReturnValue/">nursery/noRenderReturnValue</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not depend on the return value from </em><em><em>ReactDOM.render()</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noRenderReturnValue.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = ReactDOM.render(&lt;div /&gt;, document.body);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

### Valid

```jsx
ReactDOM.render(<div />, document.body);
```

