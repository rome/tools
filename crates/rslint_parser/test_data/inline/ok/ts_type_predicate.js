// TYPESCRIPT
type A = (a) => a is string;
type B = (a) => asserts a is string;
type asserts = string;
type C = () => asserts;
