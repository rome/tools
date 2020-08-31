
class Foo {
    constructor(public a: string, private b: string, protected c: string) {
        console.log(a);
        console.log(b);
        console.log(c);
    }
}
