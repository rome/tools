{ let x = 1; foo(x); }
{ for (let i in [1,2,3]) { foo(i); } }
{ for (let x of [1,2,3]) { foo(x); } }
{ let [x = -1, y] = [1,2]; y = 0; }
{ let {a: x = -1, b: y} = {a:1,b:2}; y = 0; }
{ (function() { let x = 1; foo(x); })(); }
{ (function() { for (let i in [1,2,3]) { foo(i); } })(); }
{ (function() { for (let x of [1,2,3]) { foo(x); } })(); }
{ (function() { let [x = -1, y] = [1,2]; y = 0; })(); }
{ let f = (function() { let g = x; })(); f = 1; }
{ (function() { let {a: x = -1, b: y} = {a:1,b:2}; y = 0; })(); }
{ let x = 0; { let x = 1; foo(x); } x = 0; }
{ for (let i = 0; i < 10; ++i) { let x = 1; foo(x); } }
{ for (let i in [1,2,3]) { let x = 1; foo(x); } }
{   
    var foo = function() {
        for (const b of c) {
            let a;
            a = 1;
        }
    }
}
{
    var foo = function() {
        for (const b of c) {
           let a;
           ({a} = 1);
       }
    }
}

{ let x; x = 0; }
{ switch (a) { case 0: let x; x = 0; } }
{ (function() { let x; x = 1; })(); }

{ let {a = 0, b} = obj; b = 0; foo(a, b); }
{ let {a: {b, c}} = {a: {b: 1, c: 2}}; b = 3; }
{ let {a: {b, c}} = {a: {b: 1, c: 2}} }
{ let a, b; ({a = 0, b} = obj); b = 0; foo(a, b); }
{ let {a = 0, b} = obj; foo(a, b); }
{ let [a] = [1] }
{ let {a} = obj }
{ let a, b; ({a = 0, b} = obj); foo(a, b); }
{ let {a = 0, b} = obj, c = a; b = a; }
{ let {a = 0, b} = obj, c = a; b = a; }

// https://github.com/eslint/eslint/issues/8187
{ let { name, ...otherStuff } = obj; otherStuff = {}; }
{ let { name, ...otherStuff } = obj; otherStuff = {}; }

// Warnings are located at declaration if there are reading references before assignments.
{ let x; function foo() { bar(x); } x = 0; }

// https://github.com/eslint/eslint/issues/5837
{ /*eslint use-x:error*/ let x = 1 }
{ /*eslint use-x:error*/ { let x = 1 } }
{ let { foo, bar } = baz; }

// https://github.com/eslint/eslint/issues/10520
{ const x = [1,2]; let [,y] = x; }
{ const x = [1,2,3]; let [y,,z] = x; }

// https://github.com/eslint/eslint/issues/8308
{ let predicate; [, {foo:returnType, predicate}] = foo(); }
{ let predicate; [, {foo:returnType, predicate}, ...bar ] = foo(); }
{ let predicate; [, {foo:returnType, ...predicate} ] = foo(); }
{ let x = 'x', y = 'y'; }
{ let x = 'x', y = 'y'; x = 1 }
{ let x = 1, y = 'y'; let z = 1; }
{ let { a, b, c} = obj; let { x, y, z} = anotherObj; x = 2; }
{ let x = 'x', y = 'y'; function someFunc() { let a = 1, b = 2; foo(a, b) } }

// The inner `let` will be auto-fixed in the second pass
{ let someFunc = () => { let a = 1, b = 2; foo(a, b) } }

// https://github.com/eslint/eslint/issues/11699
{ let {a, b} = c, d; }
{ let {a, b, c} = {}, e, f; }
{
    function a() {
        let foo = 0,
        bar = 1;
        foo = 1;
    }
    function b() {
        let foo = 0,
        bar = 2;
        foo = 2;
    }
}

// https://github.com/eslint/eslint/issues/13899
{ /*eslint no-undef-init:error*/ let foo = undefined; }

{ let a = 1; class C { static { a; } } }
{

    // this is a TDZ error with either `let` or `const`, but that isn't a concern of this rule
    class C { static { a; } } let a = 1;
}
{ class C { static { let a = 1; } } }
{ class C { static { if (foo) { let a = 1; } } } }
{ class C { static { let a = 1; if (foo) { a; } } } }
{ class C { static { if (foo) { let a; a = 1; } } } }
{ class C { static { let a; a = 1; } } }
{ class C { static { let { a, b } = foo; } } }
{ class C { static { let a, b; ({ a, b } = foo); } } }
{ class C { static { let a; let b; ({ a, b } = foo); } } }
{ class C { static { let a; a = 0; console.log(a); } } }

// https://github.com/eslint/eslint/issues/16266
{
    let { itemId, list } = {},
    obj = [],
    total = 0;
    total = 9;
    console.log(itemId, list, obj, total);
}
{
    let { itemId, list } = {},
    obj = [];
    console.log(itemId, list, obj);
}
{
    let [ itemId, list ] = [],
    total = 0;
    total = 9;
    console.log(itemId, list, total);
}
{
    let [ itemId, list ] = [],
    obj = [];
    console.log(itemId, list, obj);
}