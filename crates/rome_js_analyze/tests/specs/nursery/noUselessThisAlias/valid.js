const { a, b } = this;
const [c, d] = this;
const property = this.property;
const firstItem = this[0];
const object = { property: this };
foo.bar = this;

function f() {
    const self = this;
    return function distinctThisScope() {
      self.g();
    }
}

function f() {
    const self = this;
    function f() {
        self.g();
    }
    return () => {
        self.g();
    }
}

function f() {
    let self = this;
    self = {}
    return () => {
        self.g();
    }
}

function f() {
    let self;
    return () => {
        self.g();
    }
}

class Class {
    a = this;
    #priv = this;

    constructor() {
      this.b = this;
      this.c = [this];
    }

    act(self = this) {
        self.f()
    }

    f() {}
}
