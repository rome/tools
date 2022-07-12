---
title: Lint Rule noCatchAssign
layout: layouts/rule.liquid
---

# noCatchAssign

Disallow reassigning exceptions in catch clauses

## Examples

### Invalid

```jsx
try { 

} catch (e) { 
  e;
  e = 10; 
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noCatchAssign</span><span style="color: Orange;">]</span><em>: </em><em> Do not </em><em><em>reassign catch parameters.</em></em><em></em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noCatchAssign.js:3:3
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>   } catch (e) { 
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span>   e;
<span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span>   e = 10; 
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Don't re assign </span><span style="color: rgb(38, 148, 255);"><em>e</span></em><span style="color: rgb(38, 148, 255);">.</span>
<span style="color: rgb(38, 148, 255);">6</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> }
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">└</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>

=  note: Use a local variable instead.

</code></pre>{% endraw %}

### Valid

```jsx
try { 

} catch (e) { 
  let e = 10; 
  e = 100;
}
```

