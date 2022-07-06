---
title: Lint Rule useSimplifiedLogicExpression
layout: layouts/rule.liquid
---

# useSimplifiedLogicExpression

Discard redundant terms from logical expressions.

## Examples

### Invalid

```jsx
const boolExp = true;
const r = true && boolExp;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">useSimplifiedLogicExpression</span><span style="color: Orange;">]</span><em>: </em><em>Logical expression contains unnecessary complexity.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useSimplifiedLogicExpression.js:2:11
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> const r = true &amp;&amp; boolExp;
  <span style="color: rgb(38, 148, 255);">│</span>           <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0 0 |   const boolExp = true;
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const r = true &amp;&amp; boolExp;</span>
  1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const r = boolExp;</span>

</code></pre>{% endraw %}

```jsx
const boolExp2 = true;
const r2 = boolExp || true;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">useSimplifiedLogicExpression</span><span style="color: Orange;">]</span><em>: </em><em>Logical expression contains unnecessary complexity.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useSimplifiedLogicExpression.js:2:12
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> const r2 = boolExp || true;
  <span style="color: rgb(38, 148, 255);">│</span>            <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0 0 |   const boolExp2 = true;
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const r2 = boolExp || true;</span>
  1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const r2 = true;</span>

</code></pre>{% endraw %}

```jsx
const nonNullExp = 123;
const r3 = null ?? nonNullExp;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">useSimplifiedLogicExpression</span><span style="color: Orange;">]</span><em>: </em><em>Logical expression contains unnecessary complexity.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useSimplifiedLogicExpression.js:2:12
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> const r3 = null ?? nonNullExp;
  <span style="color: rgb(38, 148, 255);">│</span>            <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0 0 |   const nonNullExp = 123;
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const r3 = null ?? nonNullExp;</span>
  1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const r3 = nonNullExp;</span>

</code></pre>{% endraw %}

```jsx
const boolExpr1 = true;
const boolExpr2 = false;
const r4 = !boolExpr1 || !boolExpr2;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">useSimplifiedLogicExpression</span><span style="color: Orange;">]</span><em>: </em><em>Logical expression contains unnecessary complexity.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useSimplifiedLogicExpression.js:3:12
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> const r4 = !boolExpr1 || !boolExpr2;
  <span style="color: rgb(38, 148, 255);">│</span>            <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Reduce the complexity of the logical expression.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
0 0 |   const boolExpr1 = true;
1 1 |   const boolExpr2 = false;
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const r4 = !boolExpr1 || !boolExpr2;</span>
  2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const r4 = !(boolExpr1 &amp;&amp; boolExpr2);</span>

</code></pre>{% endraw %}

### Valid

```jsx
const boolExpr3 = true;
const boolExpr4 = false;
const r5 = !(boolExpr1 && boolExpr2);
const boolExpr5 = true;
const boolExpr6 = false;
const r6 = !!boolExpr1 || !!boolExpr2;
```

