---
title: Lint Rule useDefaultParameterLast
parent: lint/rules/index
---

# useDefaultParameterLast (since v11.0.0)

> This rule is recommended by Rome.

Enforce default function parameters and optional parameters to be last.

Default and optional parameters that precede a required parameter cannot be omitted at call site.

## Examples

### Invalid

```jsx
function f(a = 0, b) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:12 <a href="https://docs.rome.tools/lint/rules/useDefaultParameterLast">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a = 0, b) {}
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a = 0, b) {}
   <strong>   │ </strong>                  <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">A </span><span style="color: rgb(38, 148, 255);"><strong>default parameter</strong></span><span style="color: rgb(38, 148, 255);"> that precedes a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> cannot be omitted at call site.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Turn the parameter into a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a<span style="opacity: 0.8;">·</span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>b)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>             <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```jsx
function f(a, b = 0, c) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:15 <a href="https://docs.rome.tools/lint/rules/useDefaultParameterLast">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a, b = 0, c) {}
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a, b = 0, c) {}
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">A </span><span style="color: rgb(38, 148, 255);"><strong>default parameter</strong></span><span style="color: rgb(38, 148, 255);"> that precedes a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> cannot be omitted at call site.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Turn the parameter into a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a,<span style="opacity: 0.8;">·</span>b<span style="opacity: 0.8;">·</span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>c)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>

```ts
function f(a: number, b?: number, c: number) {}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:1:23 <a href="https://docs.rome.tools/lint/rules/useDefaultParameterLast">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>optional parameter</strong></span><span style="color: Tomato;"> should follow the </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a: number, b?: number, c: number) {}
   <strong>   │ </strong>                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> is here:</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function f(a: number, b?: number, c: number) {}
   <strong>   │ </strong>                                  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">A </span><span style="color: rgb(38, 148, 255);"><strong>optional parameter</strong></span><span style="color: rgb(38, 148, 255);"> that precedes a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> cannot be omitted at call site.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Turn the parameter into a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  1 │ </strong>function<span style="opacity: 0.8;">·</span>f(a:<span style="opacity: 0.8;">·</span>number,<span style="opacity: 0.8;">·</span>b<span style="color: Tomato;">?</span>:<span style="opacity: 0.8;">·</span>number,<span style="opacity: 0.8;">·</span>c:<span style="opacity: 0.8;">·</span>number)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                       <span style="color: Tomato;">-</span>                       
</code></pre>

```ts
class Foo {
    constructor(readonly a = 10, readonly b: number) {}
}
```

<pre class="language-text"><code class="language-text">style/useDefaultParameterLast.js:2:17 <a href="https://docs.rome.tools/lint/rules/useDefaultParameterLast">lint/style/useDefaultParameterLast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default parameter</strong></span><span style="color: Tomato;"> should follow the </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;"> or should be a </span><span style="color: Tomato;"><strong>required parameter</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(readonly a = 10, readonly b: number) {}
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> is here:</span>
  
    <strong>1 │ </strong>class Foo {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    constructor(readonly a = 10, readonly b: number) {}
   <strong>   │ </strong>                                 <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">A </span><span style="color: rgb(38, 148, 255);"><strong>default parameter</strong></span><span style="color: rgb(38, 148, 255);"> that precedes a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);"> cannot be omitted at call site.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Turn the parameter into a </span><span style="color: rgb(38, 148, 255);"><strong>required parameter</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>constructor(readonly<span style="opacity: 0.8;">·</span>a<span style="opacity: 0.8;">·</span><span style="color: Tomato;">=</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;">0</span>,<span style="opacity: 0.8;">·</span>readonly<span style="opacity: 0.8;">·</span>b:<span style="opacity: 0.8;">·</span>number)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>                           <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                        
</code></pre>

### Valid

```jsx
function f(a, b = 0) {}
```

```ts
function f(a: number, b?: number, c = 0) {}
```

```ts
function f(a: number, b = 0, c?: number) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
