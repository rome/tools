type A = number;
type B = string extends number ? string : number;
type C = A extends (B extends A ? number : string) ? void : number;
