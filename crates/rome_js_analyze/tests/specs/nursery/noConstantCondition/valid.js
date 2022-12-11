if (a);
if (a == 0);
if (a = f());
if (a += 1);
if (a |= 1);
if (a |= true);
if (a |= false);
if (a &= 1);
if (a &= true);
if (a &= false);
if (a >>= 1);
if (a >>= true);
if (a >>= false);
if (a >>>= 1);
if (a ??= 1);
if (a ??= true);
if (a ??= false);
if (a ||= b);
if (a ||= false);
if (a ||= 0);
if (a ||= void 0);
if (+(a ||= 1));
if (f(a ||= true));
if ((a ||= 1) + 2);
if (1 + (a ||= true));
if (a ||= '' || false);
if (a ||= void 0 || null);
if ((a ||= false) || b);
if (a || (b ||= false));
if ((a ||= true) && b);
if (a && (b ||= true));
if (a &&= b);
if (a &&= true);
if (a &&= 1);
if (a &&= 'foo');
if ((a &&= '') + false);
if ('' + (a &&= null));
if (a &&= 1 && 2);
if ((a &&= true) && b);
if (a && (b &&= true));
if ((a &&= false) || b);
if (a || (b &&= false));
if (a ||= b ||= false);
if (a &&= b &&= true);
if (a ||= b &&= false);
if (a ||= b &&= true);
if (a &&= b ||= false);
if (a &&= b ||= true);
if (1, a);
if ('every' in []);
// Ignore for now, not sure why ESLint detect this as valid
// if (`\\\n${a}`) { } 
if (`${a}`);
if (`${foo()}`);
if (`${a === 'b' && b === 'a'}`);
if (`foo${a}` === 'fooa');
if (tag`a`);
if (tag`${a}`);
if (+(a || true));
if (-(a || true));
if (~(a || 1));
if (+(a && 0) === +(b && 0));
while (~!a);
while (a = b);
while (`${a}`);
for (; x < 10;);
for (; ;);
for (; `${a}`;);
do { } while (x)
q > 0 ? 1 : 2;
`${a}` === a ? 1 : 2;
`foo${a}` === a ? 1 : 2;
tag`a` === a ? 1 : 2;
tag`${a}` === a ? 1 : 2;
while (x += 3) { };
while (tag`a`) { };
while (tag`${a}`) { };
// while (`\\\n${a}`) { }

// typeof conditions
if (typeof x === 'undefined') { };
if (`${typeof x}` === 'undefined') { };
if (a === 'str' && typeof b) { };
typeof a == typeof b;
typeof 'a' === 'string' || typeof b === 'string';
`${typeof 'a'}` === 'string' || `${typeof b}` === 'string';

// void conditions
if (void a || a);
if (a || void a);

if (xyz === 'str1' && abc === 'str2') { }
if (xyz === 'str1' || abc === 'str2') { }
if (xyz === 'str1' || abc === 'str2' && pqr === 5) { }
if (typeof abc === 'string' && abc === 'str2') { }
if (false || abc === 'str') { }
if (true && abc === 'str') { }
if (typeof 'str' && abc === 'str') { }
if (abc === 'str' || false || def === 'str') { }
if (true && abc === 'str' || def === 'str') { }
if (true && typeof abc === 'string') { }

// string literals
if ('str1' && a) { }
if (a && 'str') { }
if ((foo || true) === 'baz') { }
if ((foo || 'bar') === 'baz') { }
if ((foo || 'bar') !== 'baz') { }
if ((foo || 'bar') == 'baz') { }
if ((foo || 'bar') != 'baz') { }
if ((foo || 233) > 666) { }
if ((foo || 233) < 666) { }
if ((foo || 233) >= 666) { }
if ((foo || 233) <= 666) { }
if ((key || 'k') in obj) { }
if ((foo || {}) instanceof obj) { }
if ((foo || 'bar' || 'bar') === 'bar');

if ((foo || 1n) === 'baz') { }
if (a && 0n || b);
if (1n && a) { };

if ('' + [y] === '' + [ty]) { }
if ('a' === '' + [ty]) { }
if ('' + [y, m, d] === 'a') { }
if ('' + [y, 'm'] === '' + [ty, 'tm']) { }
if ('' + [y, 'm'] === '' + ['ty']) { }
if ([,] in ($2)); else;
if ([...x] + '' === 'y') { }

// function* foo() { while (true) { yield 'foo'; } }
// function* foo() { for (; true;) { yield 'foo'; } }
// function* foo() { do { yield 'foo'; } while (true) }
// function* foo() { while (true) { while (true) { yield; } } }
function* foo() { for (; yield;) { } }
function* foo() { for (; ; yield) { } }
// function* foo() { while (true) { function* foo() { yield; } yield; } }
function* foo() { for (let x = yield; x < 10; x++) { yield; } yield; }
function* foo() { for (let x = yield; ; x++) { yield; } }
if (new Number(x) + 1 === 2) { }

if ([a] == [b]) { }
if (+[...a]) { }
if (+[...[...a]]) { }
if (`${[...a]}`) { }
if (`${[a]}`) { }
if (+[a]) { }
if (0 - [a]) { }
if (1 * [a]) { }

// Boolean function
if (Boolean(a)) { }
if (Boolean(...args)) { }
if (foo.Boolean(1)) { }
function foo(Boolean) { if (Boolean(12)) { } }
const Boolean = () => { }; if (Boolean(1)) { }
const undefined = 'lol'; if (undefined) { }