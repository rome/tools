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
