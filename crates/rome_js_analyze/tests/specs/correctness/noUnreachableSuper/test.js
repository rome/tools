class A {
  constructor() {
    console.log("Constructing A!")
  }
}

class B extends A {
  constructor(n) {
    super();
    if (typeof n === "string") {
      if (n === "default") {
        return;
      } else {
        this.n = parseInt(n);
        return;
      }
    }
    this.n = n
  }
}

class C extends A {
  constructor(n) {
    super();
    try {} catch {}
    this.props = 0;
  }
}
