let valid: Array<Foo | Bar>;
let valid: Array<keyof Bar>;
let valid: Array<foo | bar>;

let valid5: Array<string & number>;
let valid6: Array<() => string>;
type valid7<T> = Array<T extends string ? string : number>;
type valid8 = Array<new (string, number) => string>;

//parenthesized type
let valid8: Array<string & number>;
// infer type
type valid9<T> = T extends Array<infer R> ? R : any;
// mapped type
type valid10<T> = { [K in keyof T]: T[K] };
// object type
type valid11 = Array<{ value: string; label: string }>;

let readonlyValid1: ReadonlyArray<Foo | Bar>;
let readonlyValid2: ReadonlyArray<keyof Bar>;
let readonlyValid3: ReadonlyArray<foo | bar>;
let readonlyValid4: ReadonlyArray<string & number>;
let readonlyValid5: ReadonlyArray<() => string>;
type readonlyValid6<T> = ReadonlyArray<T extends string ? string : number>;
type readonlyValid7 = ReadonlyArray<new (string, number) => string>;
let readonlyValid8: ReadonlyArray<string & number>;
type readonlyValid9<T> = T extends ReadonlyArray<infer R> ? R : any;
type readonlyValid10<T> = { [K in keyof T]: T[K] };
