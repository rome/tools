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

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:1:15 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>  const x = 5;;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove unnecessary semicolon.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>const<span style="opacity: 0.8;">·</span>x<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>5;<span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong>              <span style="color: Tomato;">-</span>
</code></pre>

```jsx
 function buzz() {
     const x = 10;;
 }    
```js,expect_diagnostic
  function foo() {
    // code
  };
```

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:4:4 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">unterminated template literal</span>
  
    <strong>2 │ </strong>     const x = 10;;
    <strong>3 │ </strong> }<span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>```js,expect_diagnostic
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  function foo() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>    // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>7 │ </strong>  };
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>8 │ </strong>
   <strong>   │ </strong>
  
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

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:2:13 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
    <strong>1 │ </strong>    class C {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>      field;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>      method() {
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove unnecessary semicolon.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>field;<span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span>
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

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:6:7 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
    <strong>4 │ </strong>     method() {
    <strong>5 │ </strong>         // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>     };
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>
    <strong>8 │ </strong>     static {
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove unnecessary semicolon.</span>
  
<strong>  </strong><strong>  6 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>}<span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong>      <span style="color: Tomato;">-</span>
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

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:10:7 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
     <strong>8 │ </strong>     static {
     <strong>9 │ </strong>         // code
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>10 │ </strong>     };
    <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>11 │ </strong>   }
    <strong>12 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove unnecessary semicolon.</span>
  
<strong>  </strong><strong>  10 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>}<span style="color: Tomato;">;</span>
<strong>  </strong><strong>     │ </strong>      <span style="color: Tomato;">-</span>
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

<pre class="language-text"><code class="language-text">nursery/noExtraSemicolons.js:11:5 <a href="https://docs.rome.tools/lint/rules/noExtraSemicolons">lint/nursery/noExtraSemicolons</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary semicolon.</span>
  
     <strong>9 │ </strong>         // code
    <strong>10 │ </strong>     }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>11 │ </strong>   };
    <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong>
    <strong>12 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove unnecessary semicolon.</span>
  
<strong>  </strong><strong>  11 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>}<span style="color: Tomato;">;</span>
<strong>  </strong><strong>     │ </strong>    <span style="color: Tomato;">-</span>
</code></pre>

