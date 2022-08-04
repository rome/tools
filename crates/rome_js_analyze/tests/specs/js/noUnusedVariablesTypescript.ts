// Invalid

class D {
	constructor(a: number) {}
	f(a: number) {}
	set a(a: number) {}
}
console.log(new D());

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
