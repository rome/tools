---
title: Lint Rule noFallthroughSwitchClause
parent: lint/rules/index
---

# noFallthroughSwitchClause (since v12.0.0)

Disallow fallthrough of case statements.

Case statements in switch statements fall through by default. This can lead to unexpected behavior when forgotten.
This rule disallows the fallthrough of case statements.

Source: https://eslint.org/docs/latest/rules/no-fallthrough

## Examples

### Invalid

```jsx
switch(bar) {
	case 0:
		a();
	case 1:
		b()
}
```

<pre class="language-text"><code class="language-text">nursery/noFallthroughSwitchClause.js:2:2 <a href="https://docs.rome.tools/lint/rules/noFallthroughSwitchClause">lint/nursery/noFallthroughSwitchClause</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This case is falling through to the next case.</span>
  
    <strong>1 │ </strong>switch(bar) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	case 0:
   <strong>   │ </strong>	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>		a();
   <strong>   │ </strong>		<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>	case 1:
    <strong>5 │ </strong>		b()
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Add a `break` or `return` statement to the end of this case to prevent fallthrough.</span>
  
</code></pre>

## Valid

```jsx
switch(foo) {
	case 1:
		doSomething();
		break;
	case 2:
		doSomething();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
