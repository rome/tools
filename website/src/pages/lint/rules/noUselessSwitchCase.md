---
title: Lint Rule noUselessSwitchCase
parent: lint/rules/index
---

# noUselessSwitchCase (since v12.0.0)

> This rule is recommended by Rome.

Disallow useless `case` in `switch` statements.

A `switch` statement can optionally have a `default` clause.

The `default` clause will be still executed only if there is no match in the `case` clauses.
An empty `case` clause that precedes the `default` clause is thus useless.

Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-useless-switch-case.md

## Examples

### Invalid

```jsx
switch (foo) {
    case 0:
    default:
        break;
    case 1:
        break;
}
```

<pre class="language-text"><code class="language-text">nursery/noUselessSwitchCase.js:2:5 <a href="https://docs.rome.tools/lint/rules/noUselessSwitchCase">lint/nursery/noUselessSwitchCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless </span><span style="color: Tomato;"><strong>case clause</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    default:
    <strong>4 │ </strong>        break;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">because the </span><span style="color: rgb(38, 148, 255);"><strong>default clause</strong></span><span style="color: rgb(38, 148, 255);"> is present:</span>
  
    <strong>1 │ </strong>switch (foo) {
    <strong>2 │ </strong>    case 0:
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    case 1:
    <strong>6 │ </strong>        break;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the useless </span><span style="color: rgb(38, 148, 255);"><strong>case</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  switch (foo) {
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>:</strong></span>
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">f</span><span style="color: Tomato;">a</span><span style="color: Tomato;">u</span><span style="color: Tomato;">l</span><span style="color: Tomato;">t</span><span style="color: Tomato;">:</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">:</span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>          break;
    <strong>5</strong> <strong>4</strong><strong> │ </strong>      case 1:
  
</code></pre>

```jsx
switch (foo) {
    default:
    case 0:
        break;
    case 1:
        break;
}
```

<pre class="language-text"><code class="language-text">nursery/noUselessSwitchCase.js:3:5 <a href="https://docs.rome.tools/lint/rules/noUselessSwitchCase">lint/nursery/noUselessSwitchCase</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Useless </span><span style="color: Tomato;"><strong>case clause</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>switch (foo) {
    <strong>2 │ </strong>    default:
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    case 1:
    <strong>6 │ </strong>        break;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">because the </span><span style="color: rgb(38, 148, 255);"><strong>default clause</strong></span><span style="color: rgb(38, 148, 255);"> is present:</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    case 0:
    <strong>4 │ </strong>        break;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the useless </span><span style="color: rgb(38, 148, 255);"><strong>case</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  switch (foo) {
    <strong>2</strong> <strong>2</strong><strong> │ </strong>      default:
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>:</strong></span>
    <strong>4</strong> <strong>3</strong><strong> │ </strong>          break;
    <strong>5</strong> <strong>4</strong><strong> │ </strong>      case 1:
  
</code></pre>

### Valid

```jsx
switch (foo) {
    case 0:
        break;
    default:
        break;
}
```

```jsx
switch (foo) {
    case 0:
        break;
}
```

