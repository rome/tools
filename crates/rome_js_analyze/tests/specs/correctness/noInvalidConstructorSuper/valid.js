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
