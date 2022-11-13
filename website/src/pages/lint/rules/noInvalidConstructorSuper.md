---
title: Lint Rule noInvalidConstructorSuper
layout: ../../../Layout.astro
---

# noInvalidConstructorSuper (since v10.0.0)

Prevents the incorrect use of `super()` inside classes.
It also checks whether a call `super()` is missing from classes that extends other constructors.

## Examples

### Invalid

```jsx
class A extends B {
    constructor() {}
}
```

<pre class="language-text"><code class="language-text">nursery/noInvalidConstructorSuper.js:1:9 <a href="https://docs.rome.tools/lint/rules/noInvalidConstructorSuper">lint/nursery/noInvalidConstructorSuper</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This class extends another class and a </span><span style="color: Orange;"><strong>super()</strong></span><span style="color: Orange;"> call is expected.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class A extends B {
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    constructor() {}
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
class A {
    constructor() {
        super();
    }
}
```

<pre class="language-text"><code class="language-text">nursery/noInvalidConstructorSuper.js:3:9 <a href="https://docs.rome.tools/lint/rules/noInvalidConstructorSuper">lint/nursery/noInvalidConstructorSuper</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This class should not have a </span><span style="color: Orange;"><strong>super()</strong></span><span style="color: Orange;"> call. You should remove it.</span>
  
    <strong>1 │ </strong>class A {
    <strong>2 │ </strong>    constructor() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        super();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    }
    <strong>5 │ </strong>}
  
</code></pre>

### Valid

```jsx
export default class A extends B {
    constructor() {
        super();
    }
}
```

```jsx
export class A {
    constructor() {}
}
```

