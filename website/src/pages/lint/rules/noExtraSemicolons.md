---
title: Lint Rule noExtraSemicolons
parent: lint/rules/index
---

# noExtraSemicolons (since v12.0.0)

Typing mistakes and misunderstandings about where semicolons are required can lead to semicolons that are unnecessary.
While not technically an error, extra semicolons can cause confusion when reading code.

This rule disallows unnecessary semicolons.

## Examples

### Invalid

```jsx
  const x = 5;;
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:1:15 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>  const x = 5;;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
  function foo() {
    // code
  };
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:3:4 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
    <strong>1 │ </strong>  function foo() {
    <strong>2 │ </strong>    // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  };
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
</code></pre>

```jsx
    class C {
      field;;

      method() {
          // code
      }

      static {
          // code
      }
    }
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:2:13 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
    <strong>1 │ </strong>    class C {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>      field;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>      method() {
  
</code></pre>

```jsx
   class C {
     field;

     method() {
         // code
     };

     static {
         // code
     }
   }
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:6:7 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
    <strong>4 │ </strong>     method() {
    <strong>5 │ </strong>         // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>     };
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>
    <strong>8 │ </strong>     static {
  
</code></pre>

```jsx
   class C {
     field;

     method() {
         // code
     }

     static {
         // code
     };
   }
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:10:7 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
     <strong>8 │ </strong>     static {
     <strong>9 │ </strong>         // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>10 │ </strong>     };
    <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>11 │ </strong>   }
    <strong>12 │ </strong>
  
</code></pre>

```jsx
   class C {
     field;

     method() {
         // code
     }

     static {
         // code
     }
   };
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:11:5 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
     <strong>9 │ </strong>         // code
    <strong>10 │ </strong>     }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>11 │ </strong>   };
    <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>12 │ </strong>
  
</code></pre>

