/* Valid  */
function case1() {
	class A { }
	foo(A);
}

function case2() {
	let A = class A { };
	foo(A);
}

function case3() {
	class A {
		b(A) {
			A = 0;
		}
	}
}

function case4() {
	class A {
		b() {
			let A;
			A = 0;
		}
	}
}

function case5() {
	let A = class {
		b() {
			A = 0;
		}
	}
}

// /* Ignores non class. */
function case6() {
	var x = 0;
	x = 1;
}

function case7() {
	let x = 0;
	x = 1;
}

function case8() {
	const x = 0;
	x = 1;
}

function case9() {
	function x() {}
	x = 1;
}

function case10(x) {
	x = 1;
}

function case11() {
	try {}
	catch (x) {
		x = 1;
	}
}

// /* Invalid  */
function case12() {
	class A { }
	A = 0;
}

function case13() {
	class B { }
	({B} = 0);
}

function case14() {
	class C { }
	({b: C = 0} = {});
}

function case15() {
	D = 0;
	class D { }
}

function case16() {
	class E {
		b() {
			E = 0;
		}
	}
}

function case17() {
	let F = class F {
		b() {
			F = 0;
		}
	}
}

function case18() {
	class G { }
	G = 0;
	G = 1;
}
