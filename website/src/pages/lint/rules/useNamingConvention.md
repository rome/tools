---
title: Lint Rule useNamingConvention
parent: lint/rules/index
---

# useNamingConvention (since vnext)

Enforce naming conventions for everything across a codebase.

Enforcing naming conventions helps keep the codebase consistent,
and reduces overhead when thinking about how to name a variable.

This rule enforces the wide-spread naming conventions of JavaScript and TypeScript codebase.

In contrast to the _ESLint_ `naming-convention` rule, this rule is not fine-tunable.
By default, an enum member has to be in _StrictUpperPascalCase_,
while the _ESLint_ requires an enum member to be in _strictCamelCase_.
This rule also supports to require an enum member to be in _CONSTANT_CASE_.

Source: https://typescript-eslint.io/rules/naming-convention/#allowed-selectors-modifiers-and-types

## Options

### strictCase

Default: _true_

By default, `HTTPServer` and `aHTTPServer` are not considered in `PascalCase` and in `camelCase`.
They have to be written as `HttpServer` and `aHttpServer`.
By setting `strictCase` to `false`, consecutive uppercase characters are allowed.

### enumMemberCase

Default: `PascalCase`

By default, the rule enforces tha naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
an `enum` member has to be in `PascalCase`.
You can enforce another convention by setting `enumMemberCase`.
The supported cases are: `camelCase`, `CONSTANT_CASE`, and `PascalCase`.

## Examples

### Invalid

```jsx
let snake_case;
```

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:5 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>top-level let</strong></span><span style="color: Tomato;"> name should be in </span><span style="color: Tomato;"><strong>camelCase</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let snake_case;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name is currently in </span><span style="color: rgb(38, 148, 255);"><strong>snake_case</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>camelCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>k</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>k</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>C</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
class camelCase {}
```

<pre class="language-text"><code class="language-text">nursery/useNamingConvention.js:1:7 <a href="https://docs.rome.tools/lint/rules/useNamingConvention">lint/nursery/useNamingConvention</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>class</strong></span><span style="color: Tomato;"> name should be in </span><span style="color: Tomato;"><strong>PascalCase</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>class camelCase {}
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The name is currently in </span><span style="color: rgb(38, 148, 255);"><strong>camelCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Rename this symbol in </span><span style="color: rgb(38, 148, 255);"><strong>PascalCase</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">l</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>C</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>C</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>m</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>C</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">}</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```jsx
let camelCase;
```

```jsx
const CONSTANT_CASE = 0;
const camelCase = {};
```

```jsx
class PascalCase {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
