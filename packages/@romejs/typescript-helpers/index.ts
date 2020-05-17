/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// rome-ignore lint/noExplicitAny
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
