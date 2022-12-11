---
title: Lint Rule noDuplicateCase
parent: lint/rules/index
---

# noDuplicateCase (since v11.0.0)

Disallow duplicate case labels.
If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.

Source: https://eslint.org/docs/latest/rules/no-duplicate-case

## Examples

### Invalid

```ts
switch (a) {
    case 1:
        break;
    case 1:
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateCase.js:4:10 <a href="https://docs.rome.tools/lint/rules/noDuplicateCase">lint/nursery/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case 1:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case 1:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
</code></pre>

```ts
switch (a) {
    case one:
        break;
    case one:
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateCase.js:4:10 <a href="https://docs.rome.tools/lint/rules/noDuplicateCase">lint/nursery/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case one:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case one:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
</code></pre>

```ts
switch (a) {
    case "1":
        break;
    case "1":
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateCase.js:4:10 <a href="https://docs.rome.tools/lint/rules/noDuplicateCase">lint/nursery/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case &quot;1&quot;:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case &quot;1&quot;:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
</code></pre>

### Valid

```ts
switch (a) {
    case 1:
        break;
    case 2:
        break;
    default:
        break;
}
```

```ts
switch (a) {
    case one:
        break;
    case two:
        break;
    default:
        break;
}
```

```ts
switch (a) {
    case "1":
        break;
    case "2":
        break;
    default:
        break;
}
```

