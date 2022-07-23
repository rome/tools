// invalid
// @ts-ignore
let foo: boolean = 1;
// @ts-ignore: Blah blah blah
let foo: boolean = 1;
/* @ts-ignore */
let foo: boolean = 1;
/** @ts-ignore */
let foo: boolean = 1;
/**
 * @ts-ignore */
let foo: boolean = 1;
/**
 ** @ts-ignore */
let foo: boolean = 1;
/**
 ** @ts-ignore
 ** @ts-ignore */
let foo2: boolean = 1;

// valid
// @ts-expect-error
let foo: boolean = 1;
// @ts-expect-error: Blah blah blah
let foo: boolean = 1;
/* @ts-expect-error */
let foo: boolean = 1;
/** @ts-expect-error */
let foo: boolean = 1;
/**
 * @ts-expect-error */
let foo: boolean = 1;
/**
 ** @ts-expect-error */
let foo: boolean = 1;
