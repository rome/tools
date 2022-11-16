const a = 1;
a = 2;

const b = 2,
	c = 43;
b = 4;
++b;
b += 45;
b--;
function f() {
	b++;
}
function f(d) {
	b++;
}
const fn = (val) => {
	val = 0;
};

const e = () => {
	try {
		foo();
	} catch (err) {
		err = 4;
	}
};

const f = (...rest) => {
	rest = 4;
};

const g = class bar {};
bar = 1;

const h = function foo() {
	foo = 1;
};
