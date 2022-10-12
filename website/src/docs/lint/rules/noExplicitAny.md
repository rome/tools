---
title: Lint Rule noExplicitAny
layout: layouts/rule.liquid
---

# noExplicitAny (since v0.10.0)

Disallow the `any` type usage

## Examples

### Invalid

```ts
let variable: any = 1;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noExplicitAny.js:1:15 <a href="https://rome.tools/docs/lint/rules/noExplicitAny">lint/nursery/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected any. Specify a different type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let variable: any = 1;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>{% endraw %}

```ts
class SomeClass {
  message: Array<Array<any>>;
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noExplicitAny.js:2:24 <a href="https://rome.tools/docs/lint/rules/noExplicitAny">lint/nursery/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected any. Specify a different type.</span>
  
    <strong>1 │ </strong>class SomeClass {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  message: Array&lt;Array&lt;any&gt;&gt;;
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
</code></pre>{% endraw %}

```ts
function fn(param: Array<any>): void {}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noExplicitAny.js:1:26 <a href="https://rome.tools/docs/lint/rules/noExplicitAny">lint/nursery/noExplicitAny</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected any. Specify a different type.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function fn(param: Array&lt;any&gt;): void {}
   <strong>   │ </strong>                         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>{% endraw %}

### Valid

```ts
let variable: number = 1;
let variable2 = 1;
```

```ts
class SomeClass {
  message: Array<Array<unknown>>;
}
```

```ts
function fn(param: Array<Array<unknown>>): Array<unknown> {}
```

```
```

