---
title: Lint Rule useButtonType
layout: layouts/rule.liquid
---

# useButtonType (since v0.10.0)

Enforces the usage of the attribute `type` for the element `button`

## Examples

### Invalid

```jsx
<button>Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><em>: </em><em>Provide an explicit </em><em><em>type</em></em><em> prop for the </em><em><em>button</em></em><em> element.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;button&gt;Do something&lt;/button&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <em>type</em> of a button is <em>submit</em>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <em>submit</em>, <em>button</em> or <em>reset</em>

</code></pre>{% endraw %}

```jsx
<button type="incorrectType">Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><em>: </em><em>Provide a valid </em><em><em>type</em></em><em> prop for the </em><em><em>button</em></em><em> element.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;button type=&quot;incorrectType&quot;&gt;Do something&lt;/button&gt;
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <em>type</em> of a button is <em>submit</em>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <em>submit</em>, <em>button</em> or <em>reset</em>

</code></pre>{% endraw %}

```jsx
React.createElement('button');
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><em>: </em><em>Provide an explicit </em><em><em>type</em></em><em> prop for the </em><em><em>button</em></em><em> element.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:21
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> React.createElement('button');
  <span style="color: rgb(38, 148, 255);">│</span>                     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <em>type</em> of a button is <em>submit</em>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <em>submit</em>, <em>button</em> or <em>reset</em>

</code></pre>{% endraw %}

## Valid

```jsx
<>
    <button type="button">Do something</button>
    <button type={buttonType}>Do something</button>
</>
```

