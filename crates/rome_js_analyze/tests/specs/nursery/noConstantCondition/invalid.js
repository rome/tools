for (; true;);
for (; ``;);
for (; `foo`;);
for (; `foo${bar}`;);
do { } while (true)
do { } while ('1')
do { } while (0)
do { } while (t = -2)
do { } while (``)
do { } while (`foo`)
do { } while (`foo${bar}`)
true ? 1 : 2;
1 ? 1 : 2;
q = 0 ? 1 : 2;
(q = 0) ? 1 : 2;
`` ? 1 : 2;
`foo` ? 1 : 2;
`foo${bar}` ? 1 : 2;
if (-2);
if (true);
if (1);
if ({});
if (0 < 1);
if (0 || 1);
if (a, 1);
if (`foo`);
if (``);
if (`\\\n`);
if (`${'bar'}`);
if (`${'bar' + `foo`}`);
if (`foo${false || true}`);
if (`foo${0 || 1}`);
if (`foo${bar}`);
if (`${bar}foo`);
if (!(true || a));
if (!(a && void b && c));
if (0 || !(a && null));
if (1 + !(a || true));
if (!(null && a) > 1);
if (+(!(a && 0)));
if (!typeof a === 'string');
if (-('foo' || a));
if (+(void a && b) === ~(1 || c));
if (a ||= true);
if (a ||= 5);
if (a ||= 'foo' || b);
if (a ||= b || /regex/);
if (a ||= b ||= true);
if (a ||= b ||= c || 1);
if (!(a ||= true));
if (!(a ||= 'foo') === true);
if (!(a ||= 'foo') === false);
if (a || (b ||= true));
if ((a ||= 1) || b);
if ((a ||= true) && true);
if (true && (a ||= true));
if (a &&= false);
if (a &&= null);
if (a &&= void b);
if (a &&= 0 && b);
if (a &&= b && '');
if (a &&= b &&= false);
if (a &&= b &&= c && false);
if (!(a &&= false));
if (!(a &&= 0) + 1);
if (a && (b &&= false));
if ((a &&= null) && b);
if (false || (a &&= false));
if ((a &&= false) || false);
while ([]);
while (~!0);
while (x = 1);
while (function () { });
while (true);
while (1);
while (() => { });
while (`foo`);
while (``);
while (`${'foo'}`);
while (`${'foo' + 'bar'}`);
if (typeof x) { }
if (typeof 'abc' === 'string') { }
if (a = typeof b) { }
if (a, typeof b) { }
if (typeof 'a' == 'string' || typeof 'b' == 'string') { }
while (typeof x) { }
if (1 || void x);
if (void x);
if (y = void x);
if (x, void x);
if (void x === void y);
if (void x && a);
if (a && void x);
if (false && abc === 'str') { }
if (true || abc === 'str') { }
if (1 || abc === 'str') { }
if (abc === 'str' || true) { }
if (abc === 'str' || true || def === 'str') { }
if (false || true) { }
if (typeof abc === 'str' || true) { }
if ('str' || a) { }
if ('str' || abc === 'str') { }
if ('str1' || 'str2') { }
if ('str1' && 'str2') { }
if (abc === 'str' || 'str') { }
if (a || 'str') { }
function* foo() { while (true) { } yield 'foo'; }
function* foo() { while (true) { if (true) { yield 'foo'; } } }
function* foo() { while (true) { yield 'foo'; } while (true) { } }
var a = function* foo() { while (true) { } yield 'foo'; }
while (true) { function* foo() { yield; } }
function* foo() { if (true) { yield 'foo'; } }
function* foo() { for (let foo = yield; true;) { } }
function* foo() { for (foo = yield; true;) { } }
function foo() { while (true) { function* bar() { while (true) { yield; } } } }
function foo() { while (true) { const bar = function* () { while (true) { yield; } } } }
function* foo() { for (let foo = 1 + 2 + 3 + (yield); true; baz) { } }
if ([a]) { }
if ([]) { }
if ('' + ['a']) { }
if ('' + []) { }
if (+1) { }
if ([,] + '') { }
if (/foo/ui);
if (0n);
if (0b0n);
if (0o0n);
if (0x0n);
if (0b1n);
if (0o1n);
if (0x1n);
if (0x1n || foo);
if (class { }) { }
if (new Foo()) { }
if (new Boolean(foo)) { }
if (new String(foo)) { }
if (new Number(foo)) { }
if (`${[...['a']]}`) { }
if (undefined) { }
if (Boolean(1)) { }
if (Boolean()) { }
if (Boolean([a])) { }
if (Boolean(1)) { function Boolean() { } }