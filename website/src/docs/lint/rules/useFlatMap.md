---
title: Lint Rule useFlatMap
layout: layouts/rule.liquid
---

# useFlatMap (since v10.0.0)

Promotes the use of `.flatMap()` when `map().flat()` are used together.

## Examples

### Invalid

```jsx
let arr = ["1", "4", ["3", "6"]];
arr.map(Number).flat();
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useFlatMap.js:2:1 <a href="https://rome.tools/docs/lint/rules/useFlatMap">lint/nursery/useFlatMap</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The call chain </span><span style="color: Orange;"><strong>.flat().map()</strong></span><span style="color: Orange;"> can be replaced with a simple </span><span style="color: Orange;"><strong>.flatMap()</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>let arr = [&quot;1&quot;, &quot;4&quot;, [&quot;3&quot;, &quot;6&quot;]];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>arr.map(Number).flat();
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with </span><span style="color: rgb(38, 148, 255);"><strong>.flatMap()</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  let arr = [&quot;1&quot;, &quot;4&quot;, [&quot;3&quot;, &quot;6&quot;]];
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">N</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>M</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">N</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>{% endraw %}

```jsx
let arr = ["1", "4", ["3", "6"]];
arr.map(Number).flat(1);
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useFlatMap.js:2:1 <a href="https://rome.tools/docs/lint/rules/useFlatMap">lint/nursery/useFlatMap</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The call chain </span><span style="color: Orange;"><strong>.flat().map()</strong></span><span style="color: Orange;"> can be replaced with a simple </span><span style="color: Orange;"><strong>.flatMap()</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>let arr = [&quot;1&quot;, &quot;4&quot;, [&quot;3&quot;, &quot;6&quot;]];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>arr.map(Number).flat(1);
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with </span><span style="color: rgb(38, 148, 255);"><strong>.flatMap()</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  let arr = [&quot;1&quot;, &quot;4&quot;, [&quot;3&quot;, &quot;6&quot;]];
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;">(</span><span style="color: Tomato;">N</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><strong>)</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>l</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>(</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>l</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>M</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">N</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>{% endraw %}

### Valid

```jsx
let arr = ["1", "4", ["3", "6"]];
arr.map(Number).flat(2);
```

