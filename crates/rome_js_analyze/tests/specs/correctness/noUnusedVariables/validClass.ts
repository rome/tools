class A {
    constructor(a: number) {
        console.log(a)
    }
    f(a: number) {
        console.log(a)
    }
}
console.log(new A(1));

// we never flag class expressions
new (class B { })

// a and b are actually properties
class C {
    constructor(private a1, public b2) {}
}
console.log(new C(1, 2));
