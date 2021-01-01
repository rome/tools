(foo++)?.();
async () => {
  (await foo)?.();
}
(+foo)?.();
class Foo extends (+Bar) {}
class Foo extends (Bar ?? Baz) {}
const foo = class extends (Bar ?? Baz) {}
