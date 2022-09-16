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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Provide an explicit </strong><strong><strong>type</strong></strong><strong> prop for the </strong><strong><strong>button</strong></strong><strong> element.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;button&gt;Do something&lt;/button&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <strong>type</strong> of a button is <strong>submit</strong>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <strong>submit</strong>, <strong>button</strong> or <strong>reset</strong>

</code></pre>{% endraw %}

```jsx
<button type="incorrectType">Do something</button>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Provide a valid </strong><strong><strong>type</strong></strong><strong> prop for the </strong><strong><strong>button</strong></strong><strong> element.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;button type=&quot;incorrectType&quot;&gt;Do something&lt;/button&gt;
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <strong>type</strong> of a button is <strong>submit</strong>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <strong>submit</strong>, <strong>button</strong> or <strong>reset</strong>

</code></pre>{% endraw %}

```jsx
React.createElement('button');
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useButtonType/">nursery/useButtonType</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Provide an explicit </strong><strong><strong>type</strong></strong><strong> prop for the </strong><strong><strong>button</strong></strong><strong> element.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/useButtonType.js:1:21
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> React.createElement('button');
  <span style="color: rgb(38, 148, 255);">│</span>                     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: The default  <strong>type</strong> of a button is <strong>submit</strong>, which causes the submission of a form when placed inside a `form` element. This is likely not the behaviour that you want inside a React application.
= <span style="color: rgb(38, 148, 255);">help</span><span style="color: rgb(38, 148, 255);">: </span>Allowed button types are: <strong>submit</strong>, <strong>button</strong> or <strong>reset</strong>

</code></pre>{% endraw %}

## Valid

```jsx
<>
    <button type="button">Do something</button>
    <button type={buttonType}>Do something</button>
</>
```

