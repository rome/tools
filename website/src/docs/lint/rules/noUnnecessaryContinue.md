---
title: Lint Rule noUnnecessaryContinue
layout: layouts/rule.liquid
---

# noUnnecessaryContinue (since v0.7.0)

> This rule is recommended by Rome.

Avoid using unnecessary `ContinueStatement`.

## Examples

### Invalid

```jsx
loop: for (let i = 0; i < 5; i++) {
  continue loop;
}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>loop: for (let i = 0; i &lt; 5; i++) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue loop;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>while (i--) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>while (1) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:4:5 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>2 │ </strong>  if (i &gt; 5) {
    <strong>3 │ </strong>    console.log(&quot;foo&quot;);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    continue;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>  } else if (i &gt;= 5 &amp;&amp; i &lt; 8) {
    <strong>6 │ </strong>    console.log(&quot;test&quot;);
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:3 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>for (let i = 0; i &lt; 9; i++) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  continue;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUnnecessaryContinue.js:2:2 <a href="https://rome.tools/docs/lint/rules/noUnnecessaryContinue">lint/correctness/noUnnecessaryContinue</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary continue statement</span>
  
    <strong>1 │ </strong>test2: do {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	continue test2;
   <strong>   │ </strong>	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>} while (true);
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Delete the unnecessary continue statement</span>
  
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

