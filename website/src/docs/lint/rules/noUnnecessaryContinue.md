---
title: Lint Rule noUnnecessaryContinue
layout: layouts/rule.liquid
---

# noUnnecessaryContinue (since v0.7.0)

Avoid using unnecessary `ContinueStatement`.

## Examples

### Invalid

```jsx
loop: for (let i = 0; i < 5; i++) {
  continue loop;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:2:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>   continue loop;
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,2 @@</span>
0 0 |   loop: for (let i = 0; i &lt; 5; i++) {
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  continue loop;</span>
2 1 |   }

</code></pre>{% endraw %}

```jsx
while (i--) {
  continue;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:2:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>   continue;
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,2 @@</span>
0 0 |   while (i--) {
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  continue;</span>
2 1 |   }

</code></pre>{% endraw %}

```jsx
while (1) {
  continue;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:2:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>   continue;
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,2 @@</span>
0 0 |   while (1) {
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  continue;</span>
2 1 |   }

</code></pre>{% endraw %}

```jsx
for (let i = 0; i < 10; i++) {
  if (i > 5) {
    console.log("foo");
    continue;
  } else if (i >= 5 && i < 8) {
    console.log("test");
  } else {
    console.log("test");
  }
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:4:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span>     continue;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,7 +1,6 @@</span>
0 0 |   for (let i = 0; i &lt; 10; i++) {
1 1 |     if (i &gt; 5) {
2 2 |       console.log(&quot;foo&quot;);
3   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    continue;</span>
4 3 |     } else if (i &gt;= 5 &amp;&amp; i &lt; 8) {
5 4 |       console.log(&quot;test&quot;);
6 5 |     } else {

</code></pre>{% endraw %}

```jsx
for (let i = 0; i < 9; i++) {
  continue;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:2:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>   continue;
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,2 @@</span>
0 0 |   for (let i = 0; i &lt; 9; i++) {
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  continue;</span>
2 1 |   }

</code></pre>{% endraw %}

```jsx
test2: do {
	continue test2;
} while (true);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue/">js/noUnnecessaryContinue</a></span><span style="color: Orange;">]</span><em>: </em><em>Unnecessary continue statement</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnnecessaryContinue.js:2:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     continue test2;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,2 @@</span>
0 0 |   test2: do {
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">	continue test2;</span>
2 1 |   } while (true);

</code></pre>{% endraw %}

### Valid

```jsx
while (i) {
  if (i > 5) {
    continue;
  }
  console.log(i);
  i--;
}

loop: while (1) {
  forLoop: for (let i = 0; i < 5; i++) {
    if (someCondition) {
      continue loop;
    }
  }
}
```

