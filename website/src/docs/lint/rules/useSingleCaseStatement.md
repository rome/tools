---
title: Lint Rule useSingleCaseStatement
layout: layouts/rule.liquid
---

# useSingleCaseStatement (since v0.7.0)

> This rule is recommended by Rome.

Enforces case clauses have a single statement, emits a quick fix wrapping
the statements in a block

## Examples

### Invalid

```jsx
switch (foo) {
    case true:
    case false:
        let foo = '';
        foo;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useSingleCaseStatement/">js/useSingleCaseStatement</a></span><span style="color: Orange;">]</span><em>: </em><em>A switch case should only have a single statement. If you want more, then wrap it in a block.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useSingleCaseStatement.js:4:9
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">┌</span>         let foo = '';
<span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span>         foo;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">└</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statements in a block</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,6 +1,7 @@</span>
0 0 |   switch (foo) {
1 1 |       case true:
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    case false:</span>
  2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">    case false: {</span>
3 3 |           let foo = '';
4 4 |           foo;
  5 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">    }</span>
5 6 |   }

</code></pre>{% endraw %}

### Valid

```jsx
switch (foo) {
    case true:
    case false: {
        let foo = '';
        foo;
    }
}
```

