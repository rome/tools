---
title: Lint Rule noShadowRestrictedNames
layout: layouts/rule.liquid
---

# noShadowRestrictedNames (since v0.8.0)

> This rule is recommended by Rome.

Disallow identifiers from shadowing restricted names.

## Examples

### Invalid

```jsx
function NaN() {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noShadowRestrictedNames/">js/noShadowRestrictedNames</a></span><span style="color: Orange;">]</span><em>: </em><em>Do not shadow the global NaN property.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noShadowRestrictedNames.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function NaN() {}
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.

</code></pre>{% endraw %}

