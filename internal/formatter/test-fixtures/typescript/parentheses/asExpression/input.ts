foo(bar as any);
while (foo as any) {}

(foo as any).bar;
(foo as any)();
class Foo extends (Bar as any) {}
(foo as number) + 1;
!(foo as any);
const bar = () => ({} as any);
(foo as any) as any;
const baz = async () => { await (foo as any); }
