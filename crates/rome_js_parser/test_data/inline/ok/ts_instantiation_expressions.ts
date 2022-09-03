let f1 = fx<string>; 
let f2 = fx<string, number>; 
let f3 = fx['test']<string>; 
const a2 = f.g<number>;  // () => number
const a3 = f<number>.g;  // <U>() => U
const a4 = f<number>.g<number>;  // () => number
const a5 = f['g']<number>;  // () => number
