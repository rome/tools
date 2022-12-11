/* should not generate diagnostics */

abstract class A {
    constructor(a: number) {
        console.log(a);
    }
    abstract f(a: number): any;
    g(a: number): any {
        console.log(a);
    }
    abstract set a(v: number);
}

class B extends A {
    constructor(a: number) {
        super(a);
    }

    f(a: number) {
        console.log(a);
    }

    g(a: number) {
        console.log(a);
    }

    set a(v: number) {
        console.log(v);
    }
}

console.log(new B(1));
