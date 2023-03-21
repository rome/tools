---
title: Lint Rule useAwait
parent: lint/rules/index
---

# useAwait (since vnext)

Disallow `async` functions that have no `await`.

Asynchronous functions in _JavaScript_ behave differently than other functions:

1. The return value is always a _Promise_.
2. You can use the `await` operator inside of them.

The primary reason to use asynchronous functions is typically to use the
`await` operator.

Asynchronous functions that don’t use `await` might not need to be asynchronous
functions and could be the unintentional result of refactoring.

Note: this rule ignores asynchronous generator functions.
This is because generators `yield` rather than `return` a value and `async` generators might
`yield` all the values of another `async` generator without ever actually needing to use `await`.

Source: https://eslint.org/docs/latest/rules/require-await

## Examples

### Invalid

```jsx
async function foo() {
    doSomething();
}
```

<pre class="language-text"><code class="language-text">nursery/useAwait.js:1:1 <a href="https://docs.rome.tools/lint/rules/useAwait">lint/nursery/useAwait</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>async</strong></span><span style="color: Orange;"> function doesn't use </span><span style="color: Orange;"><strong>await</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>async function foo() {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    doSomething();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>async</strong></span><span style="color: rgb(38, 148, 255);"> modifier or use </span><span style="color: rgb(38, 148, 255);"><strong>await</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
</code></pre>

```jsx
bar(async () => {
    doSomething();
});
```

<pre class="language-text"><code class="language-text">nursery/useAwait.js:1:5 <a href="https://docs.rome.tools/lint/rules/useAwait">lint/nursery/useAwait</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This </span><span style="color: Orange;"><strong>async</strong></span><span style="color: Orange;"> function doesn't use </span><span style="color: Orange;"><strong>await</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>bar(async () =&gt; {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    doSomething();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>});
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>async</strong></span><span style="color: rgb(38, 148, 255);"> modifier or use </span><span style="color: rgb(38, 148, 255);"><strong>await</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
</code></pre>

## Valid

```jsx
async function foo() {
    await doSomething();
}
```

```jsx
bar(async () => {
    await doSomething();
});
```

```jsx
function foo() {
    doSomething();
}
```

```jsx
bar(() => {
    doSomething();
});
```

```jsx
// Allow empty functions.
async function noop() {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
