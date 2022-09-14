---
title: Lint Rule useSelfClosingElements
layout: layouts/rule.liquid
---

# useSelfClosingElements (since v0.7.0)

> This rule is recommended by Rome.

Prevent extra closing tags for components without children

## Examples

### Invalid

```jsx
<div></div>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useSelfClosingElements/">style/useSelfClosingElements</a></span><span style="color: Tomato;">]</span><em>: </em><em>JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useSelfClosingElements.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a SelfClosingElement instead</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;div&gt;&lt;/div&gt;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;div /&gt;</span>

</code></pre>{% endraw %}

```jsx
<Component></Component>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useSelfClosingElements/">style/useSelfClosingElements</a></span><span style="color: Tomato;">]</span><em>: </em><em>JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useSelfClosingElements.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">C</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">C</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&gt;</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a SelfClosingElement instead</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;Component&gt;&lt;/Component&gt;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;Component /&gt;</span>

</code></pre>{% endraw %}

```jsx
<Foo.bar></Foo.bar>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useSelfClosingElements/">style/useSelfClosingElements</a></span><span style="color: Tomato;">]</span><em>: </em><em>JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useSelfClosingElements.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">.</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">.</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a SelfClosingElement instead</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;Foo.bar&gt;&lt;/Foo.bar&gt;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;Foo.bar /&gt;</span>

</code></pre>{% endraw %}

### Valid

```jsx
<div />
```

```jsx
<div>child</div>
```

```jsx
<Component />
```

```jsx
<Component>child</Component>
```

```jsx
<Foo.bar />
```

```jsx
<Foo.bar>child</Foo.bar>
```

