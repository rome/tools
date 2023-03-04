---
title: Lint Rule noDuplicateClassMembers
parent: lint/rules/index
---

# noDuplicateClassMembers (since v12.0.0)

Disallow duplicate class members.

If there are declarations of the same name among class members,
the last declaration overwrites other declarations silently.
It can cause unexpected behaviours.

## Examples

### Invalid

```jsx
class Foo {
  bar() { }
  bar() { }
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateClassMembers.js:3:3 <a href="https://docs.rome.tools/lint/rules/noDuplicateClassMembers">lint/nursery/noDuplicateClassMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate class member name &quot;bar&quot;</span>
  
    <strong>1 │ </strong>class Foo {
    <strong>2 │ </strong>  bar() { }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  bar() { }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
</code></pre>

```jsx
class Foo {
  bar() { }
  get bar() { }
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateClassMembers.js:3:3 <a href="https://docs.rome.tools/lint/rules/noDuplicateClassMembers">lint/nursery/noDuplicateClassMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate class member name &quot;bar&quot;</span>
  
    <strong>1 │ </strong>class Foo {
    <strong>2 │ </strong>  bar() { }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  get bar() { }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
</code></pre>

```jsx
class Foo {
  bar;
  bar() { }
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateClassMembers.js:3:3 <a href="https://docs.rome.tools/lint/rules/noDuplicateClassMembers">lint/nursery/noDuplicateClassMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate class member name &quot;bar&quot;</span>
  
    <strong>1 │ </strong>class Foo {
    <strong>2 │ </strong>  bar;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  bar() { }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
</code></pre>

```jsx
class Foo {
  static bar() { }
  static bar() { }
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateClassMembers.js:3:3 <a href="https://docs.rome.tools/lint/rules/noDuplicateClassMembers">lint/nursery/noDuplicateClassMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate class member name &quot;bar&quot;</span>
  
    <strong>1 │ </strong>class Foo {
    <strong>2 │ </strong>  static bar() { }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  static bar() { }
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
</code></pre>

## Valid

```jsx
class Foo {
  bar() { }
  qux() { }
}
```

```jsx
class Foo {
  set bar(value) { }
  get bar() { }
}
```

```jsx
class Foo {
  bar;
  qux;
}
```

```jsx
class Foo {
  bar;
  qux() { }
}
```

```jsx
class Foo {
  static bar() { }
  bar() { }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
