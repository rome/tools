var foo;
class Foo {}
!!obj?.foo
obj?.foo();
obj?.foo?.();
(obj?.foo ?? bar)();
(obj?.foo)?.()
(obj?.foo ?? bar?.baz)?.()
(obj.foo)?.();
obj?.foo.bar;
obj?.foo?.bar;
(obj?.foo)?.bar;
(obj?.foo)?.bar.baz;
(obj?.foo)?.().bar
(obj?.foo ?? bar).baz;
(obj?.foo ?? val)`template`
new (obj?.foo ?? val)()
new bar();
obj?.foo?.()();
const {foo} = obj?.baz || {};
const foo = obj?.bar
foo = obj?.bar
foo.bar = obj?.bar
bar(...obj?.foo ?? []);
var bar = {...foo?.bar};
foo?.bar in {};
foo?.bar < foo?.baz;
foo?.bar <= foo?.baz;
foo?.bar > foo?.baz;
foo?.bar >= foo?.baz;
[foo = obj?.bar] = [];
[foo.bar = obj?.bar] = [];
({foo = obj?.bar} = obj);
({foo: obj.bar = obj?.baz} = obj);
(foo?.bar, bar)();
(foo?.bar ? baz : qux)();

async function func() {
	await obj?.foo();
	await obj?.foo?.();
	(await obj?.foo)?.();
	(await obj?.foo)?.bar;
	await bar?.baz;
	await (foo ?? obj?.foo.baz);
	(await bar?.baz ?? bar).baz;
	(await bar?.baz ?? await bar).baz;
	await (foo?.bar ? baz : qux);
}

// logical operations
(obj?.foo ?? bar?.baz ?? qux)();
((obj?.foo ?? bar?.baz) || qux)();
((obj?.foo || bar?.baz) || qux)();
((obj?.foo && bar?.baz) || qux)();

// The default value option disallowArithmeticOperators is false
obj?.foo - bar;
obj?.foo + bar;
obj?.foo * bar;
obj?.foo / bar;
obj?.foo % bar;
obj?.foo ** bar;
+obj?.foo;
-obj?.foo;
bar += obj?.foo;
bar -= obj?.foo;
bar %= obj?.foo;
bar **= obj?.foo;
bar *= obj?.boo
bar /= obj?.boo
async function func() {
	await obj?.foo + await obj?.bar;
	await obj?.foo - await obj?.bar;
	await obj?.foo * await obj?.bar;
	+await obj?.foo;
	-await obj?.foo;
	bar += await obj?.foo;
	bar -= await obj?.foo;
	bar %= await obj?.foo;
	bar **= await obj?.foo;
	bar *= await obj?.boo;
	bar /= await obj?.boo;
}

obj?.foo - bar;

for (a in b?.c) {}
