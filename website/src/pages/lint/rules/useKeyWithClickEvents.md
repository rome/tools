---
title: Lint Rule useKeyWithClickEvents
parent: lint/rules/index
---

# useKeyWithClickEvents (since v10.0.0)

> This rule is recommended by Rome.

Enforce to have the `onClick` mouse event with the `onKeyUp`, the `onKeyDown`, or the `onKeyPress` keyboard event.

## Examples

### Invalid

```jsx
<div onClick={() => {}} />
```

<pre class="language-text"><code class="language-text">a11y/useKeyWithClickEvents.js:1:1 <a href="https://docs.rome.tools/lint/rules/useKeyWithClickEvents">lint/a11y/useKeyWithClickEvents</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Enforce to have the </span><span style="color: Tomato;"><strong>onClick</strong></span><span style="color: Tomato;"> mouse event with the </span><span style="color: Tomato;"><strong>onKeyUp</strong></span><span style="color: Tomato;">, the </span><span style="color: Tomato;"><strong>onKeyDown</strong></span><span style="color: Tomato;">, or the </span><span style="color: Tomato;"><strong>onKeyPress</strong></span><span style="color: Tomato;"> keyboard event.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div onClick={() =&gt; {}} /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.</span>
  
</code></pre>

```jsx
<div onClick={() => {}} ></div>
```

<pre class="language-text"><code class="language-text">a11y/useKeyWithClickEvents.js:1:1 <a href="https://docs.rome.tools/lint/rules/useKeyWithClickEvents">lint/a11y/useKeyWithClickEvents</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Enforce to have the </span><span style="color: Tomato;"><strong>onClick</strong></span><span style="color: Tomato;"> mouse event with the </span><span style="color: Tomato;"><strong>onKeyUp</strong></span><span style="color: Tomato;">, the </span><span style="color: Tomato;"><strong>onKeyDown</strong></span><span style="color: Tomato;">, or the </span><span style="color: Tomato;"><strong>onKeyPress</strong></span><span style="color: Tomato;"> keyboard event.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div onClick={() =&gt; {}} &gt;&lt;/div&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.</span>
  
</code></pre>

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

```jsx
<div onClick={() => {}} {...spread}></div>
```

```jsx
<div {...spread} onClick={() => {}} ></div>
```

```jsx
<button onClick={() => console.log("test")}>Submit</button>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
