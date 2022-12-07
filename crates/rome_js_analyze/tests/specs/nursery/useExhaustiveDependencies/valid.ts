/* should not generate diagnostics */

import useEffect from 'react';

// capturing declarations
function overloaded(s: string): string;
function overloaded(s?: string) {
  return s;
}

enum A { B = 1 }
abstract class C { static D = 1 }
class D {
    constructor() {
        
    }
}

declare module M {
    function f();
}

function MyComponent() {
    useEffect(() => {
        overloaded("");
        console.log(A.B);
        console.log(C.D);
        console.log(new D());
        console.log(M.f());
    }, []);
}