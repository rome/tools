obj?.foo | bar
obj?.foo & bar
obj?.foo >> obj?.bar;
obj?.foo << obj?.bar;
obj?.foo >>> obj?.bar;
(obj?.foo || baz) + bar;
(obj?.foo ?? baz) + bar;
(obj?.foo ?? baz) - bar;
(obj?.foo ?? baz) * bar;
(obj?.foo ?? baz) / bar;
(obj?.foo ?? baz) % bar;
(obj?.foo ?? baz) ** bar;
void obj?.foo;
typeof obj?.foo;
!obj?.foo
~obj?.foo
+(obj?.foo ?? bar)
-(obj?.foo ?? bar)
bar |= obj?.foo;
bar &= obj?.foo;
bar ^= obj?.foo;
bar <<= obj?.foo;
bar >>= obj?.foo;
bar >>>= obj?.foo;
bar ||= obj?.foo
bar &&= obj?.foo
bar += (obj?.foo ?? baz);
bar -= (obj?.foo ?? baz)
bar *= (obj?.foo ?? baz)
bar /= (obj?.foo ?? baz)
bar %= (obj?.foo ?? baz);
bar **= (obj?.foo ?? baz)

async function foo() {
	(await obj?.foo || baz) + bar;
	(await obj?.foo ?? baz) + bar;
	(await obj?.foo ?? baz) - bar;
	(await obj?.foo ?? baz) * bar;
	(await obj?.foo ?? baz) / bar;
	(await obj?.foo ?? baz) % bar;
	(await obj?.foo ?? baz) ** bar;
	void await obj?.foo;
	typeof await obj?.foo;
	!await obj?.foo
	~await obj?.foo
	+(await obj?.foo ?? bar)
	-(await obj?.foo ?? bar)
	bar |= await obj?.foo;
	bar &= await obj?.foo;
	bar ^= await obj?.foo;
	bar <<= await obj?.foo;
	bar >>= await obj?.foo;
	bar >>>= await obj?.foo
	bar += ((await obj?.foo) ?? baz);
	bar -= ((await obj?.foo) ?? baz);
	bar /= ((await obj?.foo) ?? baz);
	bar %= ((await obj?.foo) ?? baz);
	bar **= ((await obj?.foo) ?? baz);
}

obj?.foo + bar;
(foo || obj?.foo) + bar;
bar + (foo || obj?.foo);
(a ? obj?.foo : b) + bar
(a ? b : obj?.foo) + bar
(foo, bar, baz?.qux) + bar
obj?.foo - bar;
obj?.foo * bar;
obj?.foo / bar;
obj?.foo % bar;
obj?.foo ** bar;
+obj?.foo;
-obj?.foo;
+(foo ?? obj?.foo);
+(foo || obj?.bar);
+(obj?.bar && foo);
+(foo ? obj?.foo : bar);
+(foo ? bar : obj?.foo);
bar += obj?.foo;
bar -= obj?.foo;
bar %= obj?.foo;
bar **= obj?.foo;
bar *= obj?.boo
bar /= obj?.boo
bar += (foo ?? obj?.foo);
bar += (foo || obj?.foo);
bar += (foo && obj?.foo);
bar += (foo ? obj?.foo : bar);
bar += (foo ? bar : obj?.foo);
async function foo() { await obj?.foo + bar; }
async function foo() { (foo || await obj?.foo) + bar;}
async function foo() { bar + (foo || await obj?.foo); }
