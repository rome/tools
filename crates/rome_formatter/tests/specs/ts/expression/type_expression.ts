type A = string;
type B = number;
type C = null;
type D = undefined;
type E = never;
type F = unknown;
type G = object;
type H = any;
type I = boolean;
type J = bigint;
type K = symbol;
type L = void;
type M = this;
type N = "foo";
type O = true;
type P = false;
type Q = Function;
let a = 2;
type R = typeof a;
type S = 15n;
type T = -15n;
type U = 15;
type V = infer U;
type W = { a: string; b: symbol; c: symbol;d: symbol;e: symbol;f: symbol;g: symbol; };
type X = { a: string; b: symbol; }
type Z = {
    a: string
    b: symbol
}