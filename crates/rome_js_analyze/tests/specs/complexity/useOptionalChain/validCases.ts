// Do not simplify when a binary expression is used in the last expression
foo && foo.bar != null;
foo && foo.bar != undefined;
foo && foo.bar != null && baz;

//valid
foo || {};
foo || ({} as any);
(foo || { bar: 1 }).bar;
(undefined && (foo || {})).bar;
foo ||= bar;
foo ||= bar || {};
foo ||= bar?.baz;
foo ||= bar?.baz || {};
foo ||= bar?.baz?.buzz;
(foo1 ? foo2 : foo3 || {}).foo4;
(foo = 2 || {}).bar;
func(foo || {}).bar;
foo ?? {};
foo ||= bar ?? {};
foo && bar;
foo && foo;
foo || bar;
foo ?? bar;
foo || foo.bar;
foo ?? foo.bar;
file !== "index.ts" && file.endsWith(".ts");
nextToken && sourceCode.isSpaceBetweenTokens(prevToken, nextToken);
result && this.options.shouldPreserveNodeMaps;
foo && fooBar.baz;
match && match$1 !== undefined;
foo !== null && foo !== undefined;
x["y"] !== undefined && x["y"] !== null;

foo["some long"] && foo["some long string"].baz;
foo[`some long`] && foo[`some long string`].baz;
foo["some long"] && foo["some long string"].baz;
foo[123] && foo[1234].baz;
foo[true] && foo[false].baz;
foo[12n] && foo[123n].baz;
foo[/\w+/] && foo[/ab+c/].baz;

(foo || {})().bar;

(new foo() || {}).bar;
