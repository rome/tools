---
title: Lint Rule useExhaustiveDependencies
layout: layouts/docs.liquid
---

# useExhaustiveDependencies (since v10.0.0)

Enforce all dependencies are correctly specified.

## Examples

### Invalid

```jsx
let a = 1;
useEffect(() => {
    console.log(a);
})
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:2:1 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook do not specify all of its dependencies.</span>
  
    <strong>1 │ </strong>let a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>useEffect(() =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(a);
    <strong>4 │ </strong>})
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency is not specified in the hook dependency list.</span>
  
    <strong>1 │ </strong>let a = 1;
    <strong>2 │ </strong>useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    console.log(a);
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>})
    <strong>5 │ </strong>
  
</code></pre>{% endraw %}

```jsx
let b = 1;
useEffect(() => {
}, [b])
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:2:1 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook specifies more dependencies than necessary.</span>
  
    <strong>1 │ </strong>let b = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>useEffect(() =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}, [b])
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency can be removed from the list.</span>
  
    <strong>1 │ </strong>let b = 1;
    <strong>2 │ </strong>useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>}, [b])
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
</code></pre>{% endraw %}

```jsx
const [name, setName] = useState();
useEffect(() => {
    console.log(name);
    setName("");
}, [name, setName])
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:2:1 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook specifies more dependencies than necessary.</span>
  
    <strong>1 │ </strong>const [name, setName] = useState();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>useEffect(() =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    console.log(name);
    <strong>4 │ </strong>    setName(&quot;&quot;);
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency can be removed from the list.</span>
  
    <strong>3 │ </strong>    console.log(name);
    <strong>4 │ </strong>    setName(&quot;&quot;);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>}, [name, setName])
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
  
</code></pre>{% endraw %}

```jsx
let a = 1;
const b = a + 1;
useEffect(() => {
    console.log(b);
})
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:3:1 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This hook do not specify all of its dependencies.</span>
  
    <strong>1 │ </strong>let a = 1;
    <strong>2 │ </strong>const b = a + 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>useEffect(() =&gt; {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    console.log(b);
    <strong>5 │ </strong>})
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency is not specified in the hook dependency list.</span>
  
    <strong>2 │ </strong>const b = a + 1;
    <strong>3 │ </strong>useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    console.log(b);
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>})
    <strong>6 │ </strong>
  
</code></pre>{% endraw %}

## Valid

```jsx
let a = 1;
useEffect(() => {
    console.log(a);
}, [a]);
```

```jsx
const a = 1;
useEffect(() => {
    console.log(a);
});
```

```jsx
const [name, setName] = useState();
useEffect(() => {
    console.log(name);
    setName("");
}, [name])
```

