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

// valid
let valid5: Array<string & number>;
let valid6: Array<() => string>;
type valid7<T> = Array<T extends string ? string : number>
type valid8 = Array<new (string, number) => string>
// valid end

//parenthesized type
let valid8: Array<(string & number)>;
// infer type
type valid9<T> = T extends Array<infer R> ? R : any;
// mapped type
type valid10<T> = { [K in keyof T]: T[K] };

// valid
let readonlyValid1: ReadonlyArray<Foo | Bar>;
let readonlyValid2: ReadonlyArray<keyof Bar>;
let readonlyValid3: ReadonlyArray<foo | bar>;
let readonlyValid5: ReadonlyArray<string & number>;
let readonlyValid6: ReadonlyArray<() => string>;
type readonlyValid7<T> = ReadonlyArray<T extends string ? string : number>
type readonlyValid8 = ReadonlyArray<new (string, number) => string>
let readonlyValid8: ReadonlyArray<(string & number)>;
type readonlyValid9<T> = T extends ReadonlyArray<infer R> ? R : any;
type readonlyValid10<T> = { [K in keyof T]: T[K] };

// invalid
let readonlyInvalid1: ReadonlyArray<foo>;
let readonlyInvalid2: Promise<ReadonlyArray<string>>;
let readonlyInvalid3: ReadonlyArray<Foo<Bar>>;
let readonlyInvalid4: ReadonlyArray<[number, number]>;
