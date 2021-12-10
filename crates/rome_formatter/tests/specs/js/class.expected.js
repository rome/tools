class Foo extends Boar {
	constructor() {
		super();
	}
	get g() {}
	set gg(a) {}
	lorem() {
		return "ipsum";
	}
	static foo;
	static bar() {
		return "bar";
	}
	new_prop = 5;
	#a = b;
	double_semicolon = [5, 3, 4];
}
x = class {};
x = class foo extends Boar {};
x = class aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa extends bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {};
