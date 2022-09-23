---
title: Lint Rule noUnusedVariables
layout: layouts/rule.liquid
---

# noUnusedVariables (since v0.9.0)

Disallow unused variables.

There are two exceptions to this rule:

1. variables that starts with underscore, ex: `let _something;`
2. the `React` variable;

The pattern of having an underscore as prefix of a name of variable is a very diffuse
pattern among programmers, and Rome decided to follow it.

Importing the `React` variable was a mandatory pattern until some time ago:

For the time being this rule will ignore it, but this **might change in the future releases**.

## Examples

### Invalid

```jsx
const a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:7 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:7
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">a</span> = 4;
    <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
let a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:5 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let <span style="color: Tomato;">a</span> = 4;
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
function foo() {
};
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:10 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This function is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:10
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>() {
    <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
function foo(myVar) {
    console.log('foo');
}
foo();
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:14 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This parameter is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:14
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo(<span style="color: Tomato;">m</span><span style="color: Tomato;">y</span><span style="color: Tomato;">V</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span>) {
    <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
const foo = () => {
};
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:7 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:7
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span> = () =&gt; {
    <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
function foo() {
    foo();
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:10 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This function is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:10
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>() {
    <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

```jsx
const foo = () => {
    foo();
    console.log(this);
};
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUnusedVariables.js:1:7 <a href="https://rome.tools/docs/lint/rules/noUnusedVariables">lint/nursery/noUnusedVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This variable is unused.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUnusedVariables.js:1:7
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span> = () =&gt; {
    <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Unused variables usually are result of incomplete refactoring, typos and other source of bugs.</span>
  
</code></pre>{% endraw %}

# Valid

```jsx
function foo(b) {
    console.log(b)
};
foo();
```

```jsx
function foo(_unused) {
};
foo();
```

```jsx
import React from 'react';
function foo() {
    return <div />;
};
foo();
```

```ts
function used_overloaded(): number;
function used_overloaded(s: string): string;
function used_overloaded(s?: string) {
    return s;
}
used_overloaded();
```

