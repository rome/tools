/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

type VoidReturn = void | undefined;

export type VoidCallback<Args extends unknown[] = []> = Args extends []
	? ((arg?: VoidReturn) => VoidReturn)
	: ((...args: Args) => VoidReturn);

export type AsyncVoidCallback<Args extends unknown[] = []> = AsyncCallback<
	VoidReturn,
	Args
>;

export type AsyncCallback<Return, Args extends unknown[] = []> = Args extends []
	? (() => Return | Promise<Return>)
	: ((...args: Args) => Return | Promise<Return>);

export type ErrorCallback<Err extends Error = Error> = (err: Err) => void;

// rome-ignore lint/js/noUndeclaredVariables(K): don't yet support scope tracking `infer`
export type MapKey<T> = T extends Map<infer K, unknown> ? K : never;

// rome-ignore lint/js/noUndeclaredVariables(V): don't yet support scope tracking `infer`
export type MapValue<T> = T extends Map<unknown, infer V> ? V : never;

// rome-ignore lint/js/noUndeclaredVariables(V): don't yet support scope tracking `infer`
export type SetValue<T> = T extends Set<infer V> ? V : never;

// rome-ignore lint/ts/noExplicitAny lint/js/noUndeclaredVariables(I): future cleanup
export type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends ((
	k: infer I,
) => void)
	? I
	: never;

// rome-ignore lint/ts/noExplicitAny lint/js/noUndeclaredVariables(R): future cleanup
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

export type DeepPartial<T> = {[P in keyof T]?: DeepPartial<T[P]>};

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
			// @ts-ignore
			newObj[key] = a[key];
		}
	}

	return newObj;
}
