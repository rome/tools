---
title: Lint Rule useKeyWithClickEvents
layout: layouts/rule.liquid
---

# useKeyWithClickEvents (since v10.0.0)

Enforce to have the `onClick` mouse event with the `onKeyUp`, the `onKeyDown`, or the `noKeyPress` keyboard event.

## Examples

### Invalid

```jsx
<div onClick={() => {}} />
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useKeyWithClickEvents.js:1:1 <a href="https://rome.tools/docs/lint/rules/useKeyWithClickEvents">lint/nursery/useKeyWithClickEvents</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Enforce to have the </span><span style="color: Orange;"><strong>onClick</strong></span><span style="color: Orange;"> mouse event with the </span><span style="color: Orange;"><strong>onKeyUp</strong></span><span style="color: Orange;">, the </span><span style="color: Orange;"><strong>onKeyDown</strong></span><span style="color: Orange;">, or the </span><span style="color: Orange;"><strong>onKeyPress</strong></span><span style="color: Orange;"> keyboard event.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div onClick={() =&gt; {}} /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.</span>
  
</code></pre>{% endraw %}

```jsx
<div onClick={() => {}} ></div>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useKeyWithClickEvents.js:1:1 <a href="https://rome.tools/docs/lint/rules/useKeyWithClickEvents">lint/nursery/useKeyWithClickEvents</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Enforce to have the </span><span style="color: Orange;"><strong>onClick</strong></span><span style="color: Orange;"> mouse event with the </span><span style="color: Orange;"><strong>onKeyUp</strong></span><span style="color: Orange;">, the </span><span style="color: Orange;"><strong>onKeyDown</strong></span><span style="color: Orange;">, or the </span><span style="color: Orange;"><strong>onKeyPress</strong></span><span style="color: Orange;"> keyboard event.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div onClick={() =&gt; {}} &gt;&lt;/div&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
<div onClick={() => {}} onKeyDown={handleKeyDown} />
```

```jsx
<div onClick={() => {}} onKeyUp={handleKeyUp} />
```

```jsx
<div onClick={() => {}} onKeyPress={handleKeyPress} />
```

```jsx
// this rule doesn't apply to user created component
<MyComponent onClick={() => {}} />
```

