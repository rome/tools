class Foo extends Boar {
	static { // some comment
		this.a = "test";
	}
	constructor(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb, c = d) {
		super();
	}

	static get sg() {

	}

	get g() {

	}

	set gg(a) {

	}

	method() {
		return "ipsum";
	}

	async asyncMethod() {}

	* generatorMethod (){}

	static staticMethod() {
		return "bar"
	}

	async * asyncGeneratorMethod (){}

	static async staticAsyncMethod (){}

	static * staticGeneratorMethod (){}

	static async *staticAsyncGeneratorMethod() {}

	static foo;

	new_prop = 5

	#a = b

	double_semicolon = [5,3,4];;
}

x = class {
}

x = class foo extends Boar {
}

x = class aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa extends bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {
}


class A extends new A() {}
class C extends Something! {}
class B extends { a: b } {}
