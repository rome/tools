
// capturing declarations
function overloaded(): number;
function overloaded(s: string): string;
function overloaded(s?: string) {
  return s;
}

enum A { B = 1 }
abstract class C { static D = 1 }
class D {}

export type E = D;

declare module M {
    function m1();
}

function MyComponent() {
    useEffect(() => {
        overloaded();
        console.log(A.B);
        console.log(C.D);
        console.log(new E());
        console.log(m1());
    }, []);
}