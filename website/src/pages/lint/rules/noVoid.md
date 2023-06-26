---
title: Lint Rule noVoid
parent: lint/rules/index
---

# noVoid (since vnext)

Disallow the use of `void`.

>The `void` operator is often used merely to obtain the undefined primitive value,
usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.


Source: https://eslint.org/docs/latest/rules/no-void

## Examples

### Invalid

```jsx
void 0;
```

<pre class="language-text"><code class="language-text">nursery/noVoid.js:1:1 <a href="https://docs.rome.tools/lint/rules/noVoid">lint/nursery/noVoid</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The use of </span><span style="color: Orange;"><strong>void</strong></span><span style="color: Orange;"> is not allowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>void 0;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
