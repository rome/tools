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

// Ignores non class
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
