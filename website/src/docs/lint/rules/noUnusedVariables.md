---
title: Lint Rule noUnusedVariables
layout: layouts/rule.liquid
---

# noUnusedVariables (since v0.8.0)

> This rule is recommended by Rome.

Disallow unused variables.

## Examples

### Invalid

```jsx
const a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const a = 4;
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const a = 4;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
let a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let a = 4;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let a = 4;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo() {
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This function is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">function foo() {</span>
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">};</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">;</span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo(myVar) {
    console.log('foo');
}
foo();
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This parameter is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo(myVar) {
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,4 +1,4 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">function foo(myVar) {</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">function foo() {</span>
1 1 |       console.log('foo');
2 2 |   }
3 3 |   foo();

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
const foo = () => {
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = () =&gt; {
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const foo = () =&gt; {</span>
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">};</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
function foo() {
    foo();
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This function is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo() {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">function foo() {</span>
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    foo();</span>
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">}</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

```jsx
const foo = () => {
    foo();
    console.log(this);
};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnusedVariables/">js/noUnusedVariables</a></span><span style="color: Orange;">]</span><em>: </em><em>This variable is unused.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedVariables.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = () =&gt; {
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove dead code.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,4 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const foo = () =&gt; {</span>
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    foo();</span>
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    console.log(this);</span>
3   | <span style="color: Tomato;">- </span><span style="color: Tomato;">};</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

=  note: Unused variables usually are result of incomplete refactoring, typos and other source of bugs.

</code></pre>{% endraw %}

# Valid

```jsx
function foo(b) {
    console.log(b)
};
foo();
```

