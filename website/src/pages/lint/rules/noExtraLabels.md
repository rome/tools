---
title: Lint Rule noExtraLabels
parent: lint/rules/index
---

# noExtraLabels (since vnext)

Disallow unnecessary labels.

If a loop contains no nested loops or switches, labeling the loop is unnecessary.

Source: https://eslint.org/docs/latest/rules/no-extra-label

## Examples

### Invalid

```jsx
loop: while(a) {
    break loop;
}
```

<pre class="language-text"><code class="language-text">nursery/noExtraLabels.js:2:11 <a href="https://docs.rome.tools/lint/rules/noExtraLabels">lint/nursery/noExtraLabels</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>loop: while(a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    break loop;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the unnecessary </span><span style="color: rgb(38, 148, 255);"><strong>label</strong></span><span style="color: rgb(38, 148, 255);">.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">You can achieve the same result without the label.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>break<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">l</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span>;
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

### Valid

```jsx
outer: while(a) {
    while(b) {
        break outer;
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
