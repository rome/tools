---
title: Lint Rule noStaticOnlyClass
parent: lint/rules/index
---

# noStaticOnlyClass (since vnext)

This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.

Users who come from a [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming) paradigm may wrap their utility functions in an extra class,
instead of putting them at the top level of an ECMAScript module. Doing so is generally unnecessary in JavaScript and TypeScript projects.

- Wrapper classes add extra cognitive complexity to code without adding any structural improvements- Whatever would be put on them, such as utility functions, are already organized by virtue of being in a module.
- As an alternative, you can import * as ... the module to get all of them in a single object.


- IDEs can't provide as good suggestions for static class or namespace imported properties when you start typing property names
- It's more difficult to statically analyze code for unused variables, etc. when they're all on the class (see: Finding dead code (and dead types) in TypeScript).

Source: https://typescript-eslint.io/rules/no-extraneous-class

## Examples

### Invalid

```jsx
class X {
  static foo = false;
  static bar() {};
}
```

<pre class="language-text"><code class="language-text">nursery/noStaticOnlyClass.js:1:1 <a href="https://docs.rome.tools/lint/rules/noStaticOnlyClass">lint/nursery/noStaticOnlyClass</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid classes that contain only static fields.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class X {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  static foo = false;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  static bar() {};
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Prefer using simple functions instead of classes with only static fields.</span>
  
</code></pre>

```jsx
class StaticConstants {
  static readonly version = 42;

  static isProduction() {
    return process.env.NODE_ENV === 'production';
  }
}
```

<pre class="language-text"><code class="language-text">nursery/noStaticOnlyClass.js:2:10 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">'readonly' modifier can only be used in TypeScript files</span>
  
    <strong>1 │ </strong>class StaticConstants {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  static readonly version = 42;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>  static isProduction() {
  
</code></pre>

## Valid

```jsx
const X = {
  foo: false,
  bar() {}
};
```

```jsx
export const version = 42;

export function isProduction() {
  return process.env.NODE_ENV === 'production';
}

function logHelloWorld() {
  console.log('Hello, world!');
}
```

```jsx
class Empty {}
```

## Notes on Mutating Variables

One case you need to be careful of is exporting mutable variables. While class properties can be mutated externally, exported variables are always constant. This means that importers can only ever read the first value they are assigned and cannot write to the variables.

Needing to write to an exported variable is very rare and is generally considered a code smell. If you do need it you can accomplish it using getter and setter functions:

```jsx
export class Utilities {
  static mutableCount = 1;
  static incrementCount() {
    Utilities.mutableCount += 1;
  }
}
```

<pre class="language-text"><code class="language-text">nursery/noStaticOnlyClass.js:1:8 <a href="https://docs.rome.tools/lint/rules/noStaticOnlyClass">lint/nursery/noStaticOnlyClass</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid classes that contain only static fields.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>export class Utilities {
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  static mutableCount = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  static incrementCount() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    Utilities.mutableCount += 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  }
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>}
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Prefer using simple functions instead of classes with only static fields.</span>
  
</code></pre>

Do this instead:

```jsx
let mutableCount = 1;

export function getMutableCount() {
  return mutableField;
}

export function incrementCount() {
  mutableField += 1;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
