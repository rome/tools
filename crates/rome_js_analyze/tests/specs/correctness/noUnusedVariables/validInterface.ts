/* should not generate diagnostics */

interface A {
    f(a: number);
 	set a(a: number);
 	[key: string]: string;
}

class B implements A {
    f(a: number) {console.log(a)}
    set a(a: number) {console.log(a)}
    [key: string]: string;
}

console.log(new B());
