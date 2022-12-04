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
let invalid5: ReadonlyArray<string>;

// valid
let valid5: Array<string & number>;
let valid6: Array<() => string>;
type valid7<T> = Array<T extends string ? string : number>;
type valid8 = Array<new (string, number) => string>;
// valid end

//parenthesized type
let valid8: Array<string & number>;
// infer type
type valid9<T> = T extends Array<infer R> ? R : any;
// mapped type
type valid10<T> = { [K in keyof T]: T[K] };
let valid11: readonly string[];
