---
title: Lint Rule noUnusedVariables
layout: layouts/rule.liquid
---

# noUnusedVariables (since v0.9.0)

> This rule is recommended by Rome.

Disallow unused variables.

## Examples

### Invalid

```jsx
const a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">a</span> = 4;
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
let a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let <span style="color: Tomato;">a</span> = 4;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo() {
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This function is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo(myVar) {
    console.log('foo');
}
foo();
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This parameter is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo(<span style="color: Tomato;">m</span><span style="color: Tomato;">y</span><span style="color: Tomato;">V</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span>) {
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
const foo = () => {
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span> = () =&gt; {
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo() {
    foo();
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This function is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
const foo = () => {
    foo();
    console.log(this);
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Tomato;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span> = () =&gt; {
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

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

