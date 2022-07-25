---
title: Lint Rule noFunctionAssign
layout: layouts/rule.liquid
---

# noFunctionAssign (since v0.7.0)

Disallow reassigning function declarations.

## Examples

### Invalid

```jsx
function foo() { };
foo = bar;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() { };
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> foo = bar;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>

=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
function foo() {
    foo = bar;
 }
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     foo = bar;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>

=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
foo = bar;
function foo() { };
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:2:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> foo = bar;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() { };
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Reassignment happens here because the function declaration is hoisted.
=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
[foo] = bar;
function foo() { };
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:2:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> [foo] = bar;
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() { };
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Reassignment happens here because the function declaration is hoisted.
=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
({ x: foo = 0 } = bar);
function foo() { };
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:2:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> ({ x: foo = 0 } = bar);
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() { };
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Reassignment happens here because the function declaration is hoisted.
=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
function foo() {
    [foo] = bar;
 }
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     [foo] = bar;
  <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>

=  note: Use a local variable instead.

</code></pre>{% endraw %}

```jsx
(function () {
    ({ x: foo = 0 } = bar);
    function foo() { };
 })();
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noFunctionAssign/">js/noFunctionAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not reassign a function declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noFunctionAssign.js:3:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     ({ x: foo = 0 } = bar);
  <span style="color: rgb(38, 148, 255);">│</span>           <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Reassigned here.</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>     function foo() { };
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Reassignment happens here because the function declaration is hoisted.
=  note: Use a local variable instead.

</code></pre>{% endraw %}

## Valid

```jsx
function foo() {
    var foo = bar;
 }
```

```jsx
function foo(foo) {
    foo = bar;
 }
```

```jsx
function foo() {
    var foo;
    foo = bar;
 }
```

```jsx
var foo = () => {};
foo = bar;
```

```jsx
var foo = function() {};
foo = bar;
```

```jsx
var foo = function() {
    foo = bar;
 };
```

```jsx
import bar from 'bar';
function foo() {
    var foo = bar;
}
```

