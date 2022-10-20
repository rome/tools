// invalid
class A extends B{
    constructor() {
    }
}
class A extends B{
    constructor() {
        missing_super()
    }
}
class A {
    constructor() {
        super()
    }
}

class A  extends  null {
    constructor() {
        super()
    }
}
class A extends Object { constructor() { return a; } }
class A extends 'test' { constructor() { super(); } }
class A extends 100 { constructor() { super(); } }
class A extends (B = 5) { constructor() { super(); } }
class A extends (B && 5) { constructor() { super(); } }
class A extends (B &&= 5) { constructor() { super(); } }
class A extends (B += C) { constructor() { super(); } }
class A extends (B -= C) { constructor() { super(); } }
class A extends (B **= C) { constructor() { super(); } }
class A extends (B |= C) { constructor() { super(); } }
class A extends undefined { constructor() { super(); } }
// valid
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
