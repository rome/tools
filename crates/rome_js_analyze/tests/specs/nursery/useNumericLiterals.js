// Valid
parseInt(1);
parseInt(1, 3);
Number.parseInt(1);
Number.parseInt(1, 3);
0b111110111 === 503;
0o767 === 503;
0x1F7 === 503;
a[parseInt](1,2);
parseInt(foo);
parseInt(foo, 2);
Number.parseInt(foo);
Number.parseInt(foo, 2);
parseInt(11, 2);
Number.parseInt(1, 8);
parseInt(1e5, 16);
parseInt('11', '2');
Number.parseInt('11', '8');
parseInt(/foo/, 2);
parseInt(`11${foo}`, 2);
parseInt('11', 2n);
Number.parseInt('11', 8n);
parseInt('11', 16n);
parseInt(`11`, 16n);
parseInt(1n, 2);
class C { #parseInt; foo() { Number.#parseInt("111110111", 2); } }

// Invalid
parseInt("111110111", 2) === 503;
parseInt("767", 8) === 503;
parseInt("1F7", 16) === 255;
Number.parseInt("111110111", 2) === 503;
Number.parseInt("767", 8) === 503;
Number.parseInt("1F7", 16) === 255;
parseInt('7999', 8);
parseInt('1234', 2);
parseInt('1234.5', 8);
parseInt('1️⃣3️⃣3️⃣7️⃣', 16);
Number.parseInt('7999', 8);
Number.parseInt('1234', 2);
Number.parseInt('1234.5', 8);
Number.parseInt('1️⃣3️⃣3️⃣7️⃣', 16);
parseInt(`111110111`, 2) === 503;
parseInt(`767`, 8) === 503;
parseInt(`1F7`, 16) === 255;
parseInt('', 8);
parseInt(``, 8);
parseInt(`7999`, 8);
parseInt(`1234`, 2);
parseInt(`1234.5`, 8);
parseInt('11', 2)
Number.parseInt('67', 8)
5+parseInt('A', 16)
function *f(){ yield(Number).parseInt('11', 2) }
function *f(){ yield(Number.parseInt)('67', 8) }
function *f(){ yield(parseInt)('A', 16) }
function *f(){ yield Number.parseInt('11', 2) }
function *f(){ yield/**/Number.parseInt('67', 8) }
function *f(){ yield(parseInt('A', 16)) }
parseInt('11', 2)+5
Number.parseInt('17', 8)+5
parseInt('A', 16)+5
parseInt('11', 2)in foo
Number.parseInt('17', 8)in foo
parseInt('A', 16)in foo
parseInt('11', 2) in foo
Number.parseInt('17', 8)/**/in foo
(parseInt('A', 16))in foo
/* comment */Number.parseInt('11', 2);
Number/**/.parseInt('11', 2);
Number//
.parseInt('11', 2);
Number./**/parseInt('11', 2);
Number.parseInt(/**/'11', 2);
Number.parseInt('11', /**/2);
Number.parseInt('11', 2)/* comment */;
parseInt/**/('11', 2);
parseInt(//
'11', 2);
parseInt('11'/**/, 2);
parseInt(`11`/**/, 2);
parseInt('11', 2 /**/);
parseInt('11', 2)//comment
;
parseInt?.("1F7", 16) === 255;
Number?.parseInt("1F7", 16) === 255;
Number?.parseInt?.("1F7", 16) === 255;
(Number?.parseInt)("1F7", 16) === 255;
(Number?.parseInt)?.("1F7", 16) === 255;
parseInt('1_0', 2);
Number.parseInt('5_000', 8);
parseInt('0_1', 16);
Number.parseInt('0_0', 16);
