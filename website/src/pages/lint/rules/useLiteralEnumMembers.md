---
title: Lint Rule useLiteralEnumMembers
parent: lint/rules/index
---

# useLiteralEnumMembers (since v12.1.0)

Require all enum members to be literal values.

Usually, an enum member is initialized with a literal number or a literal string.
However, _TypeScript_ allows the value of an enum member to be many different kinds of expressions.
Using a computed enum member is often error-prone and confusing.
This rule requires the initialization of enum members with literal values.
It allows bitwise expressions for supporting [enum flags](https://stackoverflow.com/questions/39359740/what-are-enum-flags-in-typescript/39359953#39359953).

In contrast to the equivalent _ESLint_ rule, this rule allows arbitrary bitwise constant expressions.

Source: https://typescript-eslint.io/rules/prefer-literal-enum-member/

## Examples

### Invalid

```ts
const x = 2;
enum Computed {
    A,
    B = x,
}
```

<pre class="language-text"><code class="language-text">nursery/useLiteralEnumMembers.js:4:9 <a href="https://docs.rome.tools/lint/rules/useLiteralEnumMembers">lint/nursery/useLiteralEnumMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The enum member should be initialized with a literal value such as a number or a string.</span>
  
    <strong>2 │ </strong>enum Computed {
    <strong>3 │ </strong>    A,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    B = x,
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
</code></pre>

```ts
const x = 2;
enum Invalid {
    A,
    B = 2**3,
}
```

<pre class="language-text"><code class="language-text">nursery/useLiteralEnumMembers.js:4:9 <a href="https://docs.rome.tools/lint/rules/useLiteralEnumMembers">lint/nursery/useLiteralEnumMembers</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The enum member should be initialized with a literal value such as a number or a string.</span>
  
    <strong>2 │ </strong>enum Invalid {
    <strong>3 │ </strong>    A,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    B = 2**3,
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
</code></pre>

## Valid

```ts
enum Direction {
    Left,
    Right,
}
```

```ts
enum Order {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

```ts
enum State {
    Open = "Open",
    Close = "Close",
}
```

```ts
enum FileAccess {
    None = 0,
    Read = 1,
    Write = 1 << 1,
    All = 1 | (1 << 1)
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
