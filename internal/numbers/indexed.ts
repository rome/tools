import {MappedSet} from "@internal/collections";
import {enhanceNodeInspectClass} from "@internal/node";
import {isObject} from "@internal/typescript-helpers";

abstract class IndexedNumberBase<Super extends IndexedNumberBase<IndexedNumber>> {
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

export class ZeroIndexed extends IndexedNumberBase<ZeroIndexed> {
	constructor(value: number = 0) {
		super(value);
	}

	protected fork(value: number): ZeroIndexed {
		return new ZeroIndexed(value);
	}

	public toOneIndexed(): OneIndexed {
		return new OneIndexed(this.value + 1);
	}
}

ZeroIndexed.prototype[Symbol.toStringTag] = "ZeroIndexedNumber";

enhanceNodeInspectClass(
	ZeroIndexed,
	(inst) => {
		return `ZeroIndexedNumber<${inst.valueOf()}>`;
	},
);

export class OneIndexed extends IndexedNumberBase<OneIndexed> {
	constructor(value: number = 1) {
		super(value);
	}

	protected fork(value: number): OneIndexed {
		return new OneIndexed(value);
	}

	public toZeroIndexed(): ZeroIndexed {
		return new ZeroIndexed(this.value - 1);
	}
}

OneIndexed.prototype[Symbol.toStringTag] = "OneIndexedNumber";

enhanceNodeInspectClass(
	OneIndexed,
	(inst) => {
		return `OneIndexedNumber<${inst.valueOf()}>`;
	},
);

export class IndexedNumberSet<Indexed extends IndexedNumber>
	extends MappedSet<Indexed, number> {
	constructor(entries?: Iterable<Indexed>) {
		super((num) => [num.valueOf(), num], entries);
	}
}

IndexedNumberSet.prototype[Symbol.toStringTag] = "IndexedNumberSet";

export type IndexedNumber = OneIndexed | ZeroIndexed;

export type UnknownNumber = IndexedNumber | bigint | number;

export type IndexedNumberish = {
	[Symbol.toStringTag]: "OneIndexedNumber" | "ZeroIndexedNumber";
	valueOf: () => number;
};

// Duck typing utilities

function isIndexedNumberInstance(
	value: unknown,
	tagName: string,
): value is IndexedNumberish {
	return (
		isObject(value) &&
		// @ts-expect-error: TS does not support generic symbol indexes...
		value[Symbol.toStringTag] === tagName &&
		typeof value.valueOf === "function" &&
		typeof value.valueOf() === "number"
	);
}

export function isOneIndexedNumberish(value: unknown): value is IndexedNumberish {
	return isIndexedNumberInstance(value, "OneIndexedNumber");
}

export function isZeroIndexedNumberish(
	value: unknown,
): value is IndexedNumberish {
	return isIndexedNumberInstance(value, "ZeroIndexedNumber");
}

export function isIndexedNumberish(value: unknown): value is IndexedNumberish {
	return (
		value instanceof OneIndexed ||
		value instanceof ZeroIndexed ||
		isOneIndexedNumberish(value) ||
		isZeroIndexedNumberish(value)
	);
}
