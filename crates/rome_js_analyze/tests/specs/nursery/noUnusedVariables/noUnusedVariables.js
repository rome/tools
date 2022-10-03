import React from 'react';

const a = 1;
const b = 2,
	c = 3;
console.log(c);

function f1() {}

function f2() {
	f2();
}

function f3() {
	function g() {
		f3();
	}
	g();
}

function f41(a) {}
f41();

function f42(a, b) {
	console.log(a);
}
f42();

function f43(a, b) {
	console.log(a);
}
f43();

const f5 = () => {};

const f6 = () => {
	f6();
};

try {
} catch (e) {}

export function exported_function() {}

function exported_function_2() {}
export { exported_function_2 };

let value;
function Button() {}
console.log(<Button att={value}/>);

(function f(_a){})()

new (class C {

})