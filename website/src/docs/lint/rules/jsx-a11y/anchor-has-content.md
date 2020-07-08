---
title: Rome
layout: layouts/base.njk
showHero: false
description: Anchor elements should have contents
---

# anchor-has-content

Anchor elements should contain content that can be consumed by screen readers.
Empty anchor elements are not allowed.

## Invalid

```jsx
function Component() {
    return <a />;
}

function Component() {
    return <a></a>;
}
```

## Valid

```jsx
function Component() {
    return <a ><WrapperComponent /></a>;
}

function Component() {
    return <a dangerouslySetInnerHTML={{__html: "foo"}}></a>;
}
```

## Accessibility guidelines
- [WCAG 2.4.4](https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context)
- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
