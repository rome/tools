---
title: Lint Rule noCommaOperator
parent: lint/rules/index
---

# noCommaOperator (since v12.0.0)

> This rule is recommended by Rome.

Disallow comma operator.

The comma operator includes multiple expressions where only one is expected.
It evaluates every operand from left to right and returns the value of the last operand.
It frequently obscures side effects, and its use is often an accident.

The use of the comma operator in the initialization and update parts of a `for` is still allowed.

Source: https://eslint.org/docs/latest/rules/no-sequences

## Examples

### Invalid

```jsx
const foo = (doSomething(), 0);
```

<pre class="language-text"><code class="language-text">style/noCommaOperator.js:1:27 <a href="https://docs.rome.tools/lint/rules/noCommaOperator">lint/style/noCommaOperator</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The comma operator is disallowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const foo = (doSomething(), 0);
   <strong>   │ </strong>                          <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Its use is often confusing and obscures side effects.</span>
  
</code></pre>

```jsx
for (; doSomething(), !!test; ) {}
```

<pre class="language-text"><code class="language-text">style/noCommaOperator.js:1:21 <a href="https://docs.rome.tools/lint/rules/noCommaOperator">lint/style/noCommaOperator</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The comma operator is disallowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (; doSomething(), !!test; ) {}
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Its use is often confusing and obscures side effects.</span>
  
</code></pre>

```jsx
// Use a semicolon instead.
let a, b;
a = 1, b = 2;
```

<pre class="language-text"><code class="language-text">style/noCommaOperator.js:3:6 <a href="https://docs.rome.tools/lint/rules/noCommaOperator">lint/style/noCommaOperator</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The comma operator is disallowed.</span>
  
    <strong>1 │ </strong>// Use a semicolon instead.
    <strong>2 │ </strong>let a, b;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>a = 1, b = 2;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Its use is often confusing and obscures side effects.</span>
  
</code></pre>

### Valid

```jsx
for(a = 0, b = 0; (a + b) < 10; a++, b += 2) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
