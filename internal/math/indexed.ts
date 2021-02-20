import {enhanceNodeInspectClass} from "@internal/node";
import {isPlainObject} from "@internal/typescript-helpers";

abstract class IndexedNumber<Super extends IndexedNumber<AnyIndexedNumber>> {
	constructor(value: number) {
		this.value = value;
	}

	protected value: number;
	public [Symbol.toStringTag]: "OneIndexedNumber" | "ZeroIndexedNumber";

	public valueOf(): number {
		return this.value;
	}

	public equal(num: undefined | number | Super): boolean {
		return num !== undefined && num.valueOf() === this.value;
	}

	protected abstract fork(num: number): Super;

	public add(num: number | Super): Super {
		return this.fork(this.value + num.valueOf());
	}

	public subtract(num: number | Super): Super {
		return this.fork(this.value - num.valueOf());
	}

	public increment(): Super {
		return this.fork(this.value + 1);
	}

	public decrement(): Super {
		return this.fork(this.value - 1);
	}
}

export class ZeroIndexed extends IndexedNumber<ZeroIndexed> {
	constructor(value: number = 0) {
		super(value);
		this[Symbol.toStringTag] = "ZeroIndexedNumber";
	}

	protected fork(value: number): ZeroIndexed {
		return new ZeroIndexed(value);
	}

	public toOneIndexed(): OneIndexed {
		return new OneIndexed(this.value + 1);
	}
}

enhanceNodeInspectClass(
	ZeroIndexed,
	(inst) => {
		return `ZeroIndexedNumber<${inst.valueOf()}>`;
	},
);

export class OneIndexed extends IndexedNumber<OneIndexed> {
	constructor(value: number = 1) {
		super(value);
		this[Symbol.toStringTag] = "OneIndexedNumber";
	}

	protected fork(value: number): OneIndexed {
		return new OneIndexed(value);
	}

	public toZeroIndexed(): ZeroIndexed {
		return new ZeroIndexed(this.value - 1);
	}
}

enhanceNodeInspectClass(
	OneIndexed,
	(inst) => {
		return `OneIndexedNumber<${inst.valueOf()}>`;
	},
);

export type AnyIndexedNumber = OneIndexed | ZeroIndexed;

export type UnknownNumber = AnyIndexedNumber | bigint | number;

export type AnyIndexedNumberish = {
	[Symbol.toStringTag]: "OneIndexedNumber" | "ZeroIndexedNumber";
	valueOf: () => number;
};

// Duck typing utilities

function isIndexedNumberInstance(
	value: unknown,
	tagName: string,
): value is AnyIndexedNumberish {
	return (
		isPlainObject<{
			[Symbol.toStringTag]?: unknown;
		}>(value) &&
		value[Symbol.toStringTag] === tagName &&
		typeof value.valueOf === "function" &&
		typeof value.valueOf() === "number"
	);
}

export function isOneIndexedNumberish(
	value: unknown,
): value is AnyIndexedNumberish {
	return isIndexedNumberInstance(value, "OneIndexedNumber");
}

export function isZeroIndexedNumberish(
	value: unknown,
): value is AnyIndexedNumberish {
	return isIndexedNumberInstance(value, "ZeroIndexedNumber");
}

export function isIndexedNumberish(value: unknown): value is AnyIndexedNumberish {
	return (
		value instanceof OneIndexed ||
		value instanceof ZeroIndexed ||
		isOneIndexedNumberish(value) ||
		isZeroIndexedNumberish(value)
	);
}
