---
title: Lint Rule useAriaPropTypes
parent: lint/rules/index
---

# useAriaPropTypes (since v12.0.0)

Enforce that ARIA state and property values are valid.

## Examples

### Invalid

```jsx
<span role="checkbox" aria-checked="test" >some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:23 <a href="https://docs.rome.tools/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-checked</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span role=&quot;checkbox&quot; aria-checked=&quot;test&quot; &gt;some text&lt;/span&gt;
   <strong>   │ </strong>                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
<span aria-labelledby="" >some text</span>
```

<pre class="language-text"><code class="language-text">nursery/useAriaPropTypes.js:1:7 <a href="https://docs.rome.tools/lint/rules/useAriaPropTypes">lint/nursery/useAriaPropTypes</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The value of the ARIA attribute </span><span style="color: Orange;"><strong>aria-labelledby</strong></span><span style="color: Orange;"> is not correct.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;span aria-labelledby=&quot;&quot; &gt;some text&lt;/span&gt;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
<>
    <span role="checkbox" aria-checked={checked} >some text</span>
    <span aria-labelledby="fooId barId" >some text</span>
</>
```

## Accessibility guidelines

- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)

### Resources

- [ARIA Spec, States and Properties](https://www.w3.org/TR/wai-aria/#states_and_properties)
- [Chrome Audit Rules, AX_ARIA_04](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_04)

