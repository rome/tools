---
title: Lint Rule noConfusingLabels
parent: lint/rules/index
---

# noConfusingLabels (since v12.0.0)

> This rule is recommended by Rome.

Disallow labeled statements that are not loops.

Labeled statements in JavaScript are used in conjunction with `break` and `continue` to control flow around multiple loops.
Their use for other statements is suspicious and unfamiliar.

Source: https://eslint.org/docs/latest/rules/no-labels

## Examples

### Invalid

```jsx
label: f();
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://docs.rome.tools/lint/rules/noConfusingLabels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: f();
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Only loops should be labeled.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: {
    f();
    break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://docs.rome.tools/lint/rules/noConfusingLabels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f();
    <strong>3 │ </strong>    break label;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Only loops should be labeled.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: if (a) {
    f()
    break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://docs.rome.tools/lint/rules/noConfusingLabels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: if (a) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f()
    <strong>3 │ </strong>    break label;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Only loops should be labeled.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: switch (a) {
    case 0:
        break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://docs.rome.tools/lint/rules/noConfusingLabels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: switch (a) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    case 0:
    <strong>3 │ </strong>        break label;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Only loops should be labeled.
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

### Valid

```jsx
outer: while (a) {
    while(b) {
        break outer;
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
