foo += bar = b ??= 3;
foo -= bar;
(foo = bar);
[foo, bar] = baz;
[foo, bar = "default", ...rest] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz], foo } = {});
