/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
// This file contains generic TypeScript types, and predicate methods
type VoidReturn = void | undefined;

export type VoidCallback<Args extends unknown[] = []> = Args extends []
	? ((arg?: VoidReturn) => VoidReturn)
	: ((...args: Args) => VoidReturn);

export type DeepPartial<T> = {[P in keyof T]?: DeepPartial<T[P]>};

export type AsyncVoidCallback<Args extends unknown[] = []> = AsyncCallback<
	VoidReturn,
	Args
>;

export type AsyncCallback<Return, Args extends unknown[] = []> = Args extends []
	? (() => Return | Promise<Return>)
	: ((...args: Args) => Return | Promise<Return>);

export type ErrorCallback<Err extends Error = Error> = (err: Err) => void;

export type MapKey<T> = T extends Map<infer K, unknown> ? K : never;

export type MapValue<T> = T extends Map<unknown, infer V> ? V : never;

export type SetValue<T> = T extends Set<infer V> ? V : never;

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends ((
	k: infer I,
) => void)
	? I
	: never;

// rome-ignore lint/ts/noExplicitAny: future cleanup
type ClassConstructorParams<T> = T extends {
	new (
		...args: infer R
	): any;
}
	? R
	: never;

// rome-ignore lint/ts/noExplicitAny: future cleanup
export interface Class<T, Args extends any[] = ClassConstructorParams<T>> {
	new (
		...args: Args
	): T;
	prototype: T;
}

export type Dict<T> = Record<string, T>;

export type RequiredProps<Obj, Keys extends keyof Obj> = Omit<Obj, Keys> & {
	[Key in Keys]-?: NonNullable<Obj[Key]>
};

export type OptionalProps<Obj, Keys extends keyof Obj> = Omit<Obj, Keys> & {
	[Key in Keys]?: Obj[Key]
};

export type TaggedTemplateFunction<Ret, Sub> = (
	strs: TemplateStringsArray,
	...substitutions: Sub[]
) => Ret;

// Turn a type that contains interfaces into regular objects
export type InterfaceToObject<T> = T extends {}
	? {[K in keyof T]: InterfaceToObject<T[K]>}
	: T;

export type UnknownObject = Dict<unknown>;

export type UnknownFunction = (...args: unknown[]) => unknown;

export function isObject(obj: unknown): obj is UnknownObject {
	return typeof obj === "object" && obj !== null && !Array.isArray(obj);
}

export function isPlainObject(obj: unknown): obj is UnknownObject {
	// Weird duck typing for cross-realm objects
	return (
		isObject(obj) &&
		obj.constructor !== undefined &&
		obj.constructor.name === "Object"
	);
}

export function isIterable(obj: unknown): obj is Iterable<unknown> {
	if (typeof obj === "object" && obj != null) {
		// @ts-expect-error
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
export function mergeObjects<A extends object>(
	a: A,
	b: undefined | Partial<A>,
): A {
	if (b === undefined) {
		return a;
	}

	const newObj: A = {
		...a,
		...b,
	};

	// If b contains undefined properties then use the value from A
	for (const key in b) {
		if (b[key] === undefined) {
			newObj[key] = a[key];
		}
	}

	return newObj;
}

export function equalArray<A extends unknown[], B extends unknown[]>(
	a: A | B,
	b: B,
): a is B {
	if (a.length !== b.length) {
		return false;
	}

	if (a === b) {
		return true;
	}

	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) {
			return false;
		}
	}

	return true;
}

// Check if a value is instance of a class, disallowing inheritance
// rome-ignore lint/ts/noExplicitAny: Necessary
export function isSafeInstanceof<ClassType extends new (
	...args: any
) => any>(inst: unknown, Class: ClassType): inst is InstanceType<ClassType> {
	return inst instanceof Class && inst.constructor === Class;
}
