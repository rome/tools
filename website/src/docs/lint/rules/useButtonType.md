---
title: Lint Rule useButtonType
layout: layouts/rule.liquid
---

# useButtonType (since v0.10.0)

Enforces the usage of the attribute `type` for the element `button`

## Examples

### Invalid

```jsx
<button>Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useButtonType.js:1:1 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/nursery/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide an explicit </span><span style="color: Orange;"><strong>type</strong></span><span style="color: Orange;"> prop for the </span><span style="color: Orange;"><strong>button</strong></span><span style="color: Orange;"> element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">&gt;</span>Do something&lt;/button&gt;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The default  </span><span style="color: rgb(38, 148, 255);"><strong>type</strong></span><span style="color: rgb(38, 148, 255);"> of a button is </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowed button types are: </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, </span><span style="color: rgb(38, 148, 255);"><strong>button</strong></span><span style="color: rgb(38, 148, 255);"> or </span><span style="color: rgb(38, 148, 255);"><strong>reset</strong></span>
  
</code></pre>{% endraw %}

```jsx
<button type="incorrectType">Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useButtonType.js:1:14 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/nursery/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide a valid </span><span style="color: Orange;"><strong>type</strong></span><span style="color: Orange;"> prop for the </span><span style="color: Orange;"><strong>button</strong></span><span style="color: Orange;"> element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:14
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;button type=<span style="color: Tomato;">&quot;</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">T</span><span style="color: Tomato;">y</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span>&gt;Do something&lt;/button&gt;
    <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The default  </span><span style="color: rgb(38, 148, 255);"><strong>type</strong></span><span style="color: rgb(38, 148, 255);"> of a button is </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowed button types are: </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, </span><span style="color: rgb(38, 148, 255);"><strong>button</strong></span><span style="color: rgb(38, 148, 255);"> or </span><span style="color: rgb(38, 148, 255);"><strong>reset</strong></span>
  
</code></pre>{% endraw %}

```jsx
React.createElement('button');
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useButtonType.js:1:21 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/nursery/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide an explicit </span><span style="color: Orange;"><strong>type</strong></span><span style="color: Orange;"> prop for the </span><span style="color: Orange;"><strong>button</strong></span><span style="color: Orange;"> element.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:21
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> React.createElement(<span style="color: Tomato;">'</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">t</span><span style="color: Tomato;">t</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">'</span>);
    <span style="color: rgb(38, 148, 255);">│</span>                     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The default  </span><span style="color: rgb(38, 148, 255);"><strong>type</strong></span><span style="color: rgb(38, 148, 255);"> of a button is </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowed button types are: </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, </span><span style="color: rgb(38, 148, 255);"><strong>button</strong></span><span style="color: rgb(38, 148, 255);"> or </span><span style="color: rgb(38, 148, 255);"><strong>reset</strong></span>
  
</code></pre>{% endraw %}

## Valid

```jsx
<>
    <button type="button">Do something</button>
    <button type={buttonType}>Do something</button>
</>
```

