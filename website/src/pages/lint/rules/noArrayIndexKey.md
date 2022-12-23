---
title: Lint Rule noArrayIndexKey
parent: lint/rules/index
---

# noArrayIndexKey (since v0.10.0)

> This rule is recommended by Rome.

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

<pre class="language-text"><code class="language-text">suspicious/noArrayIndexKey.js:2:21 <a href="https://docs.rome.tools/lint/rules/noArrayIndexKey">lint/suspicious/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using the index of an array as key property in an element.</span>
  
    <strong>1 │ </strong>something.forEach((Element, index) =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    &lt;Component key={index} &gt;foo&lt;/Component&gt;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>});
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is the source of the key value.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>something.forEach((Element, index) =&gt; {
   <strong>   │ </strong>                            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    &lt;Component key={index} &gt;foo&lt;/Component&gt;
    <strong>3 │ </strong>});
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The order of the items may change, and this also affects performances and component state.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">React documentation</a></span><span style="color: rgb(38, 148, 255);">. </span>
  
</code></pre>

```jsx
React.Children.map(this.props.children, (child, index) => (
    React.cloneElement(child, { key: index })
))
```

<pre class="language-text"><code class="language-text">suspicious/noArrayIndexKey.js:2:38 <a href="https://docs.rome.tools/lint/rules/noArrayIndexKey">lint/suspicious/noArrayIndexKey</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using the index of an array as key property in an element.</span>
  
    <strong>1 │ </strong>React.Children.map(this.props.children, (child, index) =&gt; (
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    React.cloneElement(child, { key: index })
   <strong>   │ </strong>                                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>))
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is the source of the key value.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.Children.map(this.props.children, (child, index) =&gt; (
   <strong>   │ </strong>                                                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    React.cloneElement(child, { key: index })
    <strong>3 │ </strong>))
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The order of the items may change, and this also affects performances and component state.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check the </span><span style="color: rgb(38, 148, 255);"><a href="https://reactjs.org/docs/lists-and-keys.html#keys">React documentation</a></span><span style="color: rgb(38, 148, 255);">. </span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
