---
title: Lint Rule noUselessCatch
parent: lint/rules/index
---

# noUselessCatch (since vnext)

Disallow unnecessary catch clauses

## Examples

### Invalid

```jsx
try {
    doSomething();
} catch(e) {
    throw e;
}
```

<pre class="language-text"><code class="language-text">nursery/noUselessCatch.js:4:5 <a href="https://docs.rome.tools/lint/rules/noUselessCatch">lint/nursery/noUselessCatch</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The catch clause that only rethrows the original error is redundant.</span>
  
    <strong>2 │ </strong>    doSomething();
    <strong>3 │ </strong>} catch(e) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    throw e;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is recommended that these unnecessary catch clauses be removed.</span>
  
</code></pre>

```jsx
try {
    doSomething();
} catch(e) {
    throw e;
} finally {
    doCleanUp();
}
```

<pre class="language-text"><code class="language-text">nursery/noUselessCatch.js:4:5 <a href="https://docs.rome.tools/lint/rules/noUselessCatch">lint/nursery/noUselessCatch</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The catch clause that only rethrows the original error is redundant.</span>
  
    <strong>2 │ </strong>    doSomething();
    <strong>3 │ </strong>} catch(e) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    throw e;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>} finally {
    <strong>6 │ </strong>    doCleanUp();
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is recommended that these unnecessary catch clauses be removed.</span>
  
</code></pre>

## Valid

```jsx
try {
    doSomething();
} catch(e) {
    doSomethingWhenCatch();
    throw e;
}
```

```jsx
try {
    doSomething();
} catch(e) {
    handleError(e);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
