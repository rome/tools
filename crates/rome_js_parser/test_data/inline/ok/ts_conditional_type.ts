type A = number;
type B = string extends number ? string : number;
type C = A extends (B extends A ? number : string) ? void : number;
type D<T> = T extends [infer S extends string, ...unknown[]] ? S : never;
