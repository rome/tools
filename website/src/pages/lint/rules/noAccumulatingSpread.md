---
title: Lint Rule noAccumulatingSpread
parent: lint/rules/index
---

# noAccumulatingSpread (since vnext)

Disallow the use of spread (`...`) syntax on accumulators.

Spread syntax allows an iterable to be expanded into its individual elements.

Spread syntax should be avoided on accumulators (like those in `.reduce`)
because it causes a time complexity of `O(n^2)` instead of `O(n)`.

Source: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/

## Examples

### Invalid

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => [...acc, val], []);
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:25 <a href="https://docs.rome.tools/lint/rules/noAccumulatingSpread">lint/nursery/noAccumulatingSpread</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid the use of spread (`...`) syntax on accumulators.</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; [...acc, val], []);
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`.</span>
  
</code></pre>

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {return [...acc, val];}, []);
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:33 <a href="https://docs.rome.tools/lint/rules/noAccumulatingSpread">lint/nursery/noAccumulatingSpread</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid the use of spread (`...`) syntax on accumulators.</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; {return [...acc, val];}, []);
   <strong>   │ </strong>                                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`.</span>
  
</code></pre>

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {...acc, [val]: val}, {});
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:25 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">expected an expression but instead found '...acc, [val]: val'</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; {...acc, [val]: val}, {});
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Expected an expression here</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; {...acc, [val]: val}, {});
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
</code></pre>

## Valid

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {acc.push(val); return acc}, []);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
