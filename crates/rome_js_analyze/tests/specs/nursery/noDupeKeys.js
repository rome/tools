// invalid

({ a: 1, a: 2 });
({ a: 1, a: 2, a: 3 });
({ "": 1, "": 2 });
({ 0x1: 1, 1: 2 });
({ 012: 1, 10: 2 });
({ 0b1: 1, 1: 2 });
({ 0o1: 1, 1: 2 });
({ 1n: 1, 1: 2 });
({ 1_0: 1, 10: 2 });
({ "z": 1, z: 2 });
({ get a() {}, get a() {} });
({ set a(v) {}, set a(v) {} });
({ a: 1, get a() {} });
({ a: 1, set a(v) {} });
({ get a() {}, a: 1 });
({ set a(v) {}, a: 1 });
({ get a() {}, a: 1, set a(v) {} });
({ get a() {}, set a(v) {}, a: 1 });

// valid

({ a: 1, b: 1 });
({ "": 1, " ": 1 });
({ 012: 1, 12: 1 });
({ 1_0: 1, 1: 1 });
// This particular simple computed property case with just a string literal would be easy to catch,
// but we don't want to open Pandora's static analysis box so we have to draw a line somewhere
({ a: 1, ["a"]: 1 });
({ a: 1, [a]: 1 });
({ [a]: 1, [a]: 1 });
({ get a() {}, set a(v) {} });
({ a: 1, ...a });
({ a: 1, b: { a: 1, b: 1 } });
// Not object keys, so out of scope for this rule
var { a, a } = obj;
