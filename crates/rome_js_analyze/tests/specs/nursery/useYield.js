// Valid
function foo() {
	return 0;
}

function* foo() {
	yield 0;
}

function* foo() {}

(function* foo() {
	yield 0;
})();

(function* foo() {})();

const obj = {
	*foo() {
		yield 0;
	},
};

const obj = { *foo() {} };

class A {
	*foo() {
		yield 0;
	}
}

class A {
	*foo() {}
}

// Invalid
function* foo() {
	return 0;
}

(function* foo() {
	return 0;
})();

const obj = {
	*foo() {
		return 0;
	},
};

class A {
	*foo() {
		return 0;
	}
}

function* foo() {
	function* bar() {
		yield 0;
	}
}

function* foo() {
	function* bar() {
		return 0;
	}
	yield 0;
}
