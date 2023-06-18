---
title: Lint Rule noDuplicateKeys
parent: lint/rules/index
---

# noDuplicateKeys (since vnext)

Disallow two keys with the same name inside an object.

## Examples

### Invalid

```
{
  "title": "New title",
  "title": "Second title"
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateKeys.js:3:3 <a href="https://docs.rome.tools/lint/rules/noDuplicateKeys">lint/nursery/noDuplicateKeys</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The key </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> was already declared</span>
  
    <strong>1 │ </strong>{
    <strong>2 │ </strong>  &quot;title&quot;: &quot;New title&quot;,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  &quot;title&quot;: &quot;Second title&quot;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Remove or rename the key</span>
  
    <strong>1 │ </strong>{
    <strong>2 │ </strong>  &quot;title&quot;: &quot;New title&quot;,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  &quot;title&quot;: &quot;Second title&quot;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
