// valid
class B extends A {
    constructor() {
        super();
        this.field = "value";
    }
}

// valid
class C extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        } else {
            super(false);
        }

        this.field = "value";
    }
}

// invalid
class D extends A {
    constructor() {
        this.field = "value";
        super();
    }
}

// invalid
class E extends A {
    constructor(cond) {
        this.field = "value";

        if (cond) {
            super(true);
        } else {
            super(false);
        }
    }
}

// invalid
class F extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        }

        this.field = "value";
    }
}
