class Y {
    "other" = 4;
    "method"() {}
    "another_method"() {}
    "camelMethod"() {}
    "stay-like-this"() {}
    get "getter"() {}
    set "setterr"(value) {}
    "constructor"() {}
}

let value = { "a": "test", "quotes-required": "test" };

({ "a": test } = value);

let { "a": test } = value;

// you guys stay like this

({ "$$_": test } = value);

({ "%{}": test } = value);

({ "[]": test } = value);

let { "fff--fff": test } = value;

let a = { "": 10, "c_d": 30 }

let b = { "'": 10, "c_d": 30 }