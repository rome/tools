---
title: Lint Rule UseAltText
layout: layouts/rule.liquid
---

# UseAltText (since v0.10.0)

It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image.

## Examples

### Invalid

```jsx
<img src="image.png" />
```

```jsx
<input type="image" src="image.png" />
```

### Valid

```jsx
<img src="image.png" alt="image alt" />
```

```jsx
<input type="image" src="image.png" alt="alt text" />
```

