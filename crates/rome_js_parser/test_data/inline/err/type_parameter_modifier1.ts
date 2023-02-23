	export default function foo<in T>() {}
	export function foo<out T>() {}
	export function foo1<in T>() {}
	export function foo2<out T>() {}
	let foo: Foo<in T>
	let foo: Foo<out T>
	declare function foo<in T>()
	declare function foo<out T>()
	declare let foo: Foo<in T>
	declare let foo: Foo<out T>
	Foo = class <in T> {}
	Foo = class <out T> {}
	foo = function <in T>() {}
	foo = function <out T>() {}
	class Foo { foo<in T>(): T {} }
	class Foo { foo<out T>(): T {} }
	foo = { foo<in T>(): T {} };
	foo = { foo<out T>(): T {} };
	<in T>() => {};
	<out T>() => {};
	<in T, out T>() => {};
	let x: <in T>() => {};
	let x: <out T>() => {};
	let x: <in T, out T>() => {};
	let x: new <in T>() => {};
	let x: new <out T>() => {};
	let x: new <in T, out T>() => {};
	let x: { y<in T>(): any };
	let x: { y<out T>(): any };
	let x: { y<in T, out T>(): any };
