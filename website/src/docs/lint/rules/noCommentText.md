---
title: Lint Rule noCommentText
layout: layouts/rule.liquid
---

# noCommentText (since v0.7.0)

> This rule is recommended by Rome.

Prevent comments from being inserted as text nodes

## Examples

### Invalid

```jsx
const a3 = <div>// comment</div>;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noCommentText.js:1:17 <a href="https://rome.tools/docs/lint/rules/noCommentText">lint/correctness/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCommentText.js:1:17
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const a3 = &lt;div&gt;<span style="color: Tomato;">/</span><span style="color: Tomato;">/</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span>&lt;/div&gt;;
    <span style="color: rgb(38, 148, 255);">│</span>                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the comments with braces</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const a3 = &lt;div&gt;// comment&lt;/div&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const a3 = &lt;div&gt;{/* comment*/}&lt;/div&gt;;</span>
  
</code></pre>{% endraw %}

```jsx
const a4 = <div>/* comment */</div>;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noCommentText.js:1:17 <a href="https://rome.tools/docs/lint/rules/noCommentText">lint/correctness/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCommentText.js:1:17
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const a4 = &lt;div&gt;<span style="color: Tomato;">/</span><span style="color: Tomato;">*</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">*</span><span style="color: Tomato;">/</span>&lt;/div&gt;;
    <span style="color: rgb(38, 148, 255);">│</span>                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the comments with braces</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const a4 = &lt;div&gt;/* comment */&lt;/div&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const a4 = &lt;div&gt;{/* comment */}&lt;/div&gt;;</span>
  
</code></pre>{% endraw %}

```jsx
const a5 = <div>/** comment */</div>;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noCommentText.js:1:17 <a href="https://rome.tools/docs/lint/rules/noCommentText">lint/correctness/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCommentText.js:1:17
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const a5 = &lt;div&gt;<span style="color: Tomato;">/</span><span style="color: Tomato;">*</span><span style="color: Tomato;">*</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">*</span><span style="color: Tomato;">/</span>&lt;/div&gt;;
    <span style="color: rgb(38, 148, 255);">│</span>                 <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the comments with braces</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const a5 = &lt;div&gt;/** comment */&lt;/div&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const a5 = &lt;div&gt;{/* comment */}&lt;/div&gt;;</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
const a = <div>{/* comment */}</div>;
const a1 = <div>{/** comment */}</div>;
const a2 = <div className={"cls" /* comment */}></div>;
```

