switch(foo) { case 1: doSomething(); break; case 2: doSomething(); }

function bar(foo) { switch(foo) { case 1: doSomething(); return; case 2: doSomething(); } }

switch(foo) { case 1: doSomething(); throw new Error("Boo!"); case 2: doSomething(); }

switch(foo) { case 1: case 2: doSomething(); }

switch(foo) { case 1: { doSomething(); break; } case 2: doSomething(); }
