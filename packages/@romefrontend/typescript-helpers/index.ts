/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// rome-ignore lint/js/noExplicitAny
export type Class<T, Args extends Array<unknown> = Array<any>> = {
	new (
		...args: Args
	): T;
};

export type Dict<T> = {
	[key: string]: T;
};

export type RequiredProps<Obj, Keys extends keyof Obj> = Omit<Obj, Keys> & {
	[Key in Keys]-?: NonNullable<Obj[Key]>
};

export type OptionalProps<Obj, Keys extends keyof Obj> = Omit<Obj, Keys> & {
	[Key in Keys]?: Obj[Key]
};

// Turn a type that contains interfaces into regular objects
export type InterfaceToObject<T> = T extends {}
	? {[K in keyof T]: InterfaceToObject<T[K]>}
	: T;

export type UnknownObject = Dict<unknown>;

export function isPlainObject<T = UnknownObject>(
	obj: unknown,
): obj is UnknownObject & T {
	return typeof obj === "object" && obj !== null && !Array.isArray(obj);
}

export function isIterable(obj: unknown): obj is Iterable<unknown> {
	if (typeof obj === "object" && obj != null) {
		// @ts-ignore
		return typeof obj[Symbol.iterator] === "function";
	} else {
		return false;
	}
}

// TypeScript has awful behaviour where it treats optional properties and undefined values as the same
// eg. https://www.typescriptlang.org/play/index.html?ssl=3&ssc=1&pln=1&pc=1#code/C4TwDgpgBAChBOBnA9gOygXigbwFBQKgDMBLJYAOQEMBbCALikWHhNQHMBufQgGyubU6jZqw7cAvt1wBjNMyhgq8YCSq84SNIzyEo-QbQgB+ESzbsANLgmYcPAgcpHGAV1QATCKVQQP1qVxZeWBFBBRURk0Iu11CUnIhBigAIgBlCAAjAVUqVBTrPSckxhSKdxAqAocoADp6pRU1DXC0AOkAeg79HKSoEkQody8fPyhM11CAFTSoD2QIRFQAclCEeGR4XEgtVFrio0wMLGHvNj9OIA
// https://github.com/microsoft/TypeScript/issues/13195
// This means that places where we expect to receive a value when object spreading in a partial object
// can actually have undefined values!
export function mergeObjects<A extends object, B extends Partial<A>>(
	a: A,
	b: B,
): A & B {
	const newObj: A & B = {
		...a,
		...b,
	};

	// If b contains undefined properties then use the value from A
	for (const key in b) {
		if (b[key] === undefined) {
			// @ts-ignore
			newObj[key] = a[key];
		}
	}

	return newObj;
}
