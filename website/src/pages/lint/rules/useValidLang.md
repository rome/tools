---
title: Lint Rule useValidLang
parent: lint/rules/index
---

# useValidLang (since v12.0.0)

Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.

## Examples

### Invalid

```jsx
<html lang="lorem" />
```

<pre class="language-text"><code class="language-text">nursery/useValidLang.js:1:12 <a href="https://docs.rome.tools/lint/rules/useValidLang">lint/nursery/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;lorem&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Some of valid languages:</span>
  
  - ab
  - aa
  - af
  - sq
  - am
  - ar
  - an
  - hy
  - as
  - ay
  - az
  - ba
  - eu
  - bn
  - dz
  
</code></pre>

```jsx
<html lang="en-babab" />
```

<pre class="language-text"><code class="language-text">nursery/useValidLang.js:1:12 <a href="https://docs.rome.tools/lint/rules/useValidLang">lint/nursery/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;en-babab&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Some of valid countries:</span>
  
  - AF
  - AL
  - DZ
  - AS
  - AD
  - AO
  - AI
  - AQ
  - AG
  - AR
  - AM
  - AW
  - AU
  - AT
  - AZ
  
</code></pre>

```jsx
<html lang="en-GB-typo" />
```

<pre class="language-text"><code class="language-text">nursery/useValidLang.js:1:12 <a href="https://docs.rome.tools/lint/rules/useValidLang">lint/nursery/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;en-GB-typo&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
<Html lang="en-babab" />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
