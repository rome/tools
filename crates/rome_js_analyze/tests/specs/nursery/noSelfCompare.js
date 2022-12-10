// valid
if (a || b) { }
if (1 ^ 2) { }
if (x === y) { }
if (1 === 2) { }
y=x*x
foo.bar.baz === foo.bar.qux
class C { #field; foo() { this.#field === this['#field']; } }
class C { #field; foo() { this['#field'] === this.#field; } }

// invalid
if (x === x) { }
if (x !== x) { }
if (x > x) { }
if ('x' > 'x') { }
if ('x' > "x") { }
do {} while (x === x)
x === x
x !== x
x == x
x != x
x > x
x < x
x >= x
x <= x
foo.bar().baz.qux >= foo.bar ().baz .qux
class C { #field; foo() { this.#field === this.#field; } }
