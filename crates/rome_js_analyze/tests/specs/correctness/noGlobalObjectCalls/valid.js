var x = Math;
var x = Math.random();
var x = Math.PI;
var x = foo.Math();
var x = new foo.Math();
var x = new Math.foo;
var x = new Math.foo();
JSON.parse(foo);
new JSON.parse;
Reflect.get(foo, 'x');
new Reflect.foo(a, b);
Atomics.load(foo, 0);
new Atomics.foo();
new Intl.Segmenter();
Intl.foo();

globalThis.Math.random();
var x = globalThis.Math.PI;
f(globalThis.Math.log(1));
globalThis.Math.log2().foo;
var x = globalThis.JSON.parse();
x = globalThis.JSON.stringify(str);
globalThis.Math.exp( globalThis.JSON.parse(1) );
var x = globalThis.Reflect.get(123);
/*globals Reflect: true*/ globalThis.Reflect.get();
var x = globalThis.Atomics.load([]);
var x = globalThis.Intl.Segmenter();

// shadowed variables
var Math; Math();
var Math; new Math();
let JSON; JSON();
let JSON; new JSON();
if (foo) { const Reflect = 1; Reflect(); }
if (foo) { const Reflect = 1; new Reflect(); }
function foo(Math) { Math(); }
function foo(JSON) { new JSON(); }
function foo(Atomics) { Atomics(); }
function foo() { if (bar) { let Atomics; if (baz) { new Atomics(); } } }
function foo() { var JSON; JSON(); }
function foo() { var Atomics = bar(); var baz = Atomics(5); }
var construct = typeof Reflect !== "undefined" ? Reflect.construct : undefined; construct();
function foo(Intl) { Intl(); }
if (foo) { const Intl = 1; Intl(); }
if (foo) { const Intl = 1; new Intl(); }
