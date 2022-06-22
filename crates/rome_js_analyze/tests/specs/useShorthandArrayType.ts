// valid
let valid: Array<Foo | Bar>;
let valid: Array<keyof Bar>;
let valid: Array<foo | bar>;
// invalid
let valid: Array<foo>;
let invalid1: Array<foo, Array<string>>;
let invalid2: Promise<Array<string>>;
let invalid3: Array<Foo<Bar>>;
let invalid4: Array<[number, number]>;

