---
title: Lint Rule useExhaustiveDependencies
parent: lint/rules/index
---

# useExhaustiveDependencies (since v10.0.0)

Enforce all dependencies are correctly specified.

## Examples

### Invalid

```jsx
function component() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    });
}
```

<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:3:5 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook do not specify all of its dependencies.</span>
  
    <strong>1 │ </strong>function component() {
    <strong>2 │ </strong>    let a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>        console.log(a);
    <strong>5 │ </strong>    });
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency is not specified in the hook dependency list.</span>
  
    <strong>2 │ </strong>    let a = 1;
    <strong>3 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        console.log(a);
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    });
    <strong>6 │ </strong>}
  
</code></pre>

```jsx
function component() {
    let b = 1;
    useEffect(() => {
    }, [b]);
}
```

<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:3:5 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook specifies more dependencies than necessary.</span>
  
    <strong>1 │ </strong>function component() {
    <strong>2 │ </strong>    let b = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }, [b]);
    <strong>5 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency can be removed from the list.</span>
  
    <strong>2 │ </strong>    let b = 1;
    <strong>3 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    }, [b]);
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
</code></pre>

```jsx
function component() {
    const [name, setName] = useState();
    useEffect(() => {
        console.log(name);
        setName("");
    }, [name, setName]);
}
```

<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:3:5 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook specifies more dependencies than necessary.</span>
  
    <strong>1 │ </strong>function component() {
    <strong>2 │ </strong>    const [name, setName] = useState();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>        console.log(name);
    <strong>5 │ </strong>        setName(&quot;&quot;);
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency can be removed from the list.</span>
  
    <strong>4 │ </strong>        console.log(name);
    <strong>5 │ </strong>        setName(&quot;&quot;);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    }, [name, setName]);
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
</code></pre>

```jsx
function component() {
    let a = 1;
    const b = a + 1;
    useEffect(() => {
        console.log(b);
    });
}
```

<pre class="language-text"><code class="language-text">nursery/useExhaustiveDependencies.js:4:5 <a href="https://docs.rome.tools/lint/rules/useExhaustiveDependencies">lint/nursery/useExhaustiveDependencies</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This hook do not specify all of its dependencies.</span>
  
    <strong>2 │ </strong>    let a = 1;
    <strong>3 │ </strong>    const b = a + 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    useEffect(() =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        console.log(b);
    <strong>6 │ </strong>    });
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This dependency is not specified in the hook dependency list.</span>
  
    <strong>3 │ </strong>    const b = a + 1;
    <strong>4 │ </strong>    useEffect(() =&gt; {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        console.log(b);
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>    });
    <strong>7 │ </strong>}
  
</code></pre>

## Valid

```jsx
function component() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    }, [a]);
}
```

```jsx
function component() {
    const a = 1;
    useEffect(() => {
        console.log(a);
    });
}
```

```jsx
function component() {
    const [name, setName] = useState();
    useEffect(() => {
        console.log(name);
        setName("");
    }, [name]);
}
```

## Options

Allows to specify custom hooks - from libraries or internal projects - that can be considered stable.

```
{
    "//": "...",
    "options": {
        "hooks": [
            { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
            { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
        ]
    }
}
```

Given the previous example, your hooks be used like this:

```jsx
function Foo() {
    const location = useLocation(() => {}, []);
    const query = useQuery([], () => {});
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
