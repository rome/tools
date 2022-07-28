---
title: Lint Rule noAsyncPromiseExecutor
layout: layouts/rule.liquid
---

# noAsyncPromiseExecutor (since v0.7.0)

> This rule is recommended by Rome.

Disallows using an async function as a Promise executor.

The executor function can also be an async function. However, this is usually a mistake, for a few reasons:

1. If an async executor function throws an error, the error will be lost and won't cause the newly-constructed `Promise` to reject. This could make it difficult to debug and handle some errors.
2. If a Promise executor function is using `await`, this is usually a sign that it is not actually necessary to use the `new Promise` constructor, or the scope of the `new Promise` constructor can be reduced.
## Examples

### Invalid

```jsx
new Promise(async function foo(resolve, reject) {})
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noAsyncPromiseExecutor/">js/noAsyncPromiseExecutor</a></span><span style="color: Tomato;">]</span><em>: </em><em>Promise executor functions should not be `async`.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noAsyncPromiseExecutor.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> new Promise(<span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">y</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;"> </span><span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;"> </span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">(</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">v</span><span style="color: Tomato;">e</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">j</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>)
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

```jsx
  new Promise(async (resolve, reject) => {})
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noAsyncPromiseExecutor/">js/noAsyncPromiseExecutor</a></span><span style="color: Tomato;">]</span><em>: </em><em>Promise executor functions should not be `async`.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noAsyncPromiseExecutor.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>   new Promise(<span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">y</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">v</span><span style="color: Tomato;">e</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">j</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">=</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>)
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

```jsx
  new Promise(((((async () => {})))))
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noAsyncPromiseExecutor/">js/noAsyncPromiseExecutor</a></span><span style="color: Tomato;">]</span><em>: </em><em>Promise executor functions should not be `async`.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noAsyncPromiseExecutor.js:1:19
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>   new Promise(((((<span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">y</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">=</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>)))))
  <span style="color: rgb(38, 148, 255);">│</span>                   <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

### Valid

```jsx
  new Promise((resolve, reject) => {})
  new Promise((resolve, reject) => {}, async function unrelated() {})
  new Foo(async (resolve, reject) => {})
  new Foo((( (resolve, reject) => {} )))
```

