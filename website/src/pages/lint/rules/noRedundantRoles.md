---
title: Lint Rule noRedundantRoles
parent: lint/rules/index
---

# noRedundantRoles (since vnext)

Enforce explicit `role` property is not the same as implicit/default role property on an element.

EsLint Equivalent: [no-redundant-roles](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md)

## Examples

### Invalid

```jsx
<article role='article'></article>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:15 <a href="https://docs.rome.tools/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'article' on the 'article' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;article role='article'&gt;&lt;/article&gt;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> 
  
</code></pre>

```jsx
<button role='button'></button>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:14 <a href="https://docs.rome.tools/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'button' on the 'button' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;button role='button'&gt;&lt;/button&gt;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> 
  
</code></pre>

```jsx
<h1 role='heading' aria-level='1'>title</h1>
```

<pre class="language-text"><code class="language-text">nursery/noRedundantRoles.js:1:10 <a href="https://docs.rome.tools/lint/rules/noRedundantRoles">lint/nursery/noRedundantRoles</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Using the role attribute 'heading' on the 'h1' element is redundant.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;h1 role='heading' aria-level='1'&gt;title&lt;/h1&gt;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> 
  
</code></pre>

## Valid

```jsx
<article role='presentation'></article>
```

```jsx
<Button role='button'></Button>
```

```jsx
<span></span>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
