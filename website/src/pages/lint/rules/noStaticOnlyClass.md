---
title: Lint Rule noStaticOnlyClass
parent: lint/rules/index
---

# noStaticOnlyClass (since vnext)

Succinct description of the rule.

Put context and details about the rule.
As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).

Try to stay consistent with the descriptions of implemented rules.

Add a link to the corresponding ESLint rule (if any):

Source: https://eslint.org/docs/latest/rules/rule-name

## Examples

### Invalid

```jsx
class X {
  static foo = false;
  static bar() {};
}
```

<pre class="language-text"><code class="language-text">nursery/noStaticOnlyClass.js:1:7 <a href="https://docs.rome.tools/lint/rules/noStaticOnlyClass">lint/nursery/noStaticOnlyClass</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid classes that contain only static fields</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class X {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>  static foo = false;
    <strong>3 │ </strong>  static bar() {};
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Prefer using simple functions instead of classes with only static fields</span>
  
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

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
