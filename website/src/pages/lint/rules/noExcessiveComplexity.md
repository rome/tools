---
title: Lint Rule noExcessiveComplexity
parent: lint/rules/index
---

# noExcessiveComplexity (since vnext)

The more complexity a function contains, the harder it is to understand
later on.

Reducing complexity helps to make code more maintenable, both by making
it easier to understand as well as by reducing chances of accidental
side-effects when making changes.

This rule calculates a complexity score for every function and signals
those that exceed a configured complexity threshold (default: 10).

Sources:

- https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md
- https://eslint.org/docs/latest/rules/complexity (note this rule uses "cyclomatic complexity" instead)

## Examples

### Invalid

```jsx
function tooComplex() {
    for (let x = 0; x < 10; x++) {
        for (let y = 0; y < 10; y++) {
            if (x % 2 === 0) {
                if (y % 2 === 0) {
                    console.log(x > y ? `${x} > ${y}` : `${y} > ${x}`);
                }
            }
        }
    }
}
```

<pre class="language-text"><code class="language-text">nursery/noExcessiveComplexity.js:1:10 <a href="https://docs.rome.tools/lint/rules/noExcessiveComplexity">lint/nursery/noExcessiveComplexity</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Excessive complexity detected.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function tooComplex() {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    for (let x = 0; x &lt; 10; x++) {
    <strong>3 │ </strong>        for (let y = 0; y &lt; 10; y++) {
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Please refactor this code to reduce its complexity from 15 to 10.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
