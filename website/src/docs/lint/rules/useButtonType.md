---
title: Lint Rule useButtonType
layout: layouts/rule.liquid
---

# useButtonType (since v0.10.0)

> This rule is recommended by Rome.

Enforces the usage of the attribute `type` for the element `button`

## Examples

### Invalid

```jsx
<button>Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useButtonType.js:1:1 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/a11y/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide an explicit </span><span style="color: Tomato;"><strong>type</strong></span><span style="color: Tomato;"> prop for the </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;button&gt;Do something&lt;/button&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The default  </span><span style="color: rgb(38, 148, 255);"><strong>type</strong></span><span style="color: rgb(38, 148, 255);"> of a button is </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowed button types are: </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, </span><span style="color: rgb(38, 148, 255);"><strong>button</strong></span><span style="color: rgb(38, 148, 255);"> or </span><span style="color: rgb(38, 148, 255);"><strong>reset</strong></span>
  
</code></pre>{% endraw %}

```jsx
<button type="incorrectType">Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useButtonType.js:1:14 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/a11y/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid </span><span style="color: Tomato;"><strong>type</strong></span><span style="color: Tomato;"> prop for the </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;button type=&quot;incorrectType&quot;&gt;Do something&lt;/button&gt;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The default  </span><span style="color: rgb(38, 148, 255);"><strong>type</strong></span><span style="color: rgb(38, 148, 255);"> of a button is </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowed button types are: </span><span style="color: rgb(38, 148, 255);"><strong>submit</strong></span><span style="color: rgb(38, 148, 255);">, </span><span style="color: rgb(38, 148, 255);"><strong>button</strong></span><span style="color: rgb(38, 148, 255);"> or </span><span style="color: rgb(38, 148, 255);"><strong>reset</strong></span>
  
</code></pre>{% endraw %}

```jsx
React.createElement('button');
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useButtonType.js:1:21 <a href="https://rome.tools/docs/lint/rules/useButtonType">lint/a11y/useButtonType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide an explicit </span><span style="color: Tomato;"><strong>type</strong></span><span style="color: Tomato;"> prop for the </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.createElement('button');
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
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

