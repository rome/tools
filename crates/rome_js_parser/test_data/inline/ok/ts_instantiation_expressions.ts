let f1 = fx<string>; 
let f2 = fx<string, number>; 
let f3 = fx['test']<string>; 
const a2 = f.g<number>;  // () => number
const a3 = f<number>.g;  // <U>() => U
const a4 = f<number>.g<number>;  // () => number
const a5 = f['g']<number>;  // () => number
const a7 = (f<number>)['g'];
const a6 = f<number>['g'];  // type Error
const b2 = f?.<number>();
// const b3 = f<number>?.();
// const b4 = f<number>?.<number>();  // Type Error, expected no type arguments
