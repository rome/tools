declare const SOMECONSTANT: number;

declare class D {
	constructor(a: number);
	f(a: number);
	set a(a: number);
}

declare function unused_overloaded(): number;
declare function unused_overloaded(s: string): string;
declare function unused_overloaded(s?: string);

type Command = (...args: any[]) => unknown;

declare module Module { 
    class A { 
       f(b:number) : number; 
    }
    const B;
 }