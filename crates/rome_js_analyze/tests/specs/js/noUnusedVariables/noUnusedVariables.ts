// Invalid

class D {
	constructor(a: number) {}
	f(a: number) {}
	set a(a: number) {}
}
console.log(new D());

function unused_overloaded(): number;
function unused_overloaded(s: string): string;
function unused_overloaded(s?: string) {
  return s;
}

// Valid

interface A {
	f(a: number);
	set a(a: number);
	[key: string]: string;
}

abstract class B {
	constructor(a: number);
	abstract f(a: number);
	g(a: number);
	abstract set a(a: number);
}
console.log(new B());

class C {
	constructor(a: number);
	f(a: number);
}
console.log(new C());

function f(fn: (title: string) => boolean) {
	console.log(fn);
}
f();

export type Command = (...args: any[]) => unknown;

function used_overloaded(): number;
function used_overloaded(s: string): string;
function used_overloaded(s?: string) {
  return s;
}
used_overloaded();
