class A  extends B {
    constructor() {
        super()
    }
}

class A extends (B &&= C) { constructor() { super(); } }

class A extends (false && B) { constructor() { super(); } }

class A extends (B, C) { constructor() { super(); } }

class A extends null { constructor() { return a; } }

class A extends Object { constructor() { super() } }

class A extends (5 && B) { constructor() { super(); } }

class A extends (B || C) { constructor() { super(); } }

module.exports = class A extends B {
    constructor() {
        super();
    }
}

export class A extends B {
    constructor() {
        super();
    }
}

export default class A extends B {
    constructor() {
        super();
    }
}

export class A extends mod.B {
    constructor() {
        super();
    }
}

// Regression test for https://github.com/rome/tools/issues/4624
class ExtendGeneric
    extends A<number>
    implements I {

        constructor() {
		super();
	}
}

class ExtendTaggesTemplate extends tag`something` {
	constructor() {
		super();
	}
}

class ExtendUntaggedTemplate extends `something` {
	constructor() {}
}

class ExtendNullAssertion extends A! {
    constructor() {
		super();
	}
}

class ExtendTypeAssertion extends (A as A) {
    constructor() {
		super();
	}
}

class ExtendStatisfiesExpression extends (A satisfies A) {
    constructor() {
		super();
	}
}
