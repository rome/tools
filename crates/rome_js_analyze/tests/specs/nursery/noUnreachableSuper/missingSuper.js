// valid
class A {
    constructor() {}
}

// valid
class B extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        } else {
            super(false);
        }
    }
}

// invalid
class C extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        }
    }
}

// invalid
class D extends A {
    constructor(variant) {
        switch (variant) {
            case 0:
                break;
            default:
                super();
                break;
        }
    }
}

// invalid
class E extends A {
    constructor(cond) {
        if (cond) {
            return;
        }

        super(true);
    }
}

// valid
class F extends A {
    constructor(variant) {
        switch (variant) {
            case 0:
            default:
                super();
                break;
        }
    }
}

// valid
class G extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        } else {
            throw new Error();
        }

        this.field = "value";
    }
}
