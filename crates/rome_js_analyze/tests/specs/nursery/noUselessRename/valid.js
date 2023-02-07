let { foo } = obj;

let { foo: bar } = obj;

let { foo: bar, baz: qux } = obj;

let {
	foo: { bar: baz },
} = obj;

let {
	foo,
	bar: { baz: qux },
} = obj;

let { foo: bar } = obj;

let { foo: bar, baz: qux } = obj;

let {
	foo: { bar: baz },
} = obj;

let {
	foo,
	bar: { baz: qux },
} = obj;

let { ["foo"]: bar } = obj;

let { ["foo"]: bar, ["baz"]: qux } = obj;

let {
	["foo"]: { ["bar"]: baz },
} = obj;

let {
	foo,
	["bar"]: { ["baz"]: qux },
} = obj;

let { [foo]: foo } = obj;

let { ["foo"]: foo } = obj;

let { [foo]: bar } = obj;

function func({ foo }) {}

function func({ foo: bar }) {}

function func({ foo: bar, baz: qux }) {}

({ foo }) => {};

({ foo: bar }) => {};

({ foo: bar, baz: qui }) => {};

import * as foo from "foo";

import foo from "foo";

import { foo } from "foo";

import { foo as bar } from "foo";

import { foo as bar, baz as qux } from "foo";

import { "foo" as bar } from "baz";

export { foo } from "foo";

var foo = 0;
export { foo as bar };

var foo = 0;
var baz = 0;
export { foo as bar, baz as qux };

export { foo as bar } from "foo";

export { foo as bar, baz as qux } from "foo";

var foo = 0;
export { foo as "bar" };

export { foo as "bar" } from "baz";

export { "foo" as bar } from "baz";

export { "foo" as "bar" } from "baz";

export { "" as " " } from "baz";

export { " " as "" } from "baz";

export { "foo" } from "bar";

const { ...stuff } = myObject;

const { foo, ...stuff } = myObject;

const { foo: bar, ...stuff } = myObject;
