import {enhanceNodeInspectClass} from "@internal/node";
import {Resource, createResourceFromCallback} from "@internal/resources";

type DurationOptions = {
	approx?: boolean;
	decimal?: number;
	value: bigint;
};

type DurationFromBigIntOptions = Omit<DurationOptions, "value">;
type DurationFromNumberOptions = Omit<DurationOptions, "value" | "precision">;

type BigIntWithDecimal = {
	value: bigint;
	decimal: number;
};

function convertToMultipliedBigInt(
	num: bigint | number,
	factorNum: number,
	factorBig: bigint,
): BigIntWithDecimal {
	if (typeof num === "bigint") {
		return {
			value: num * factorBig,
			decimal: 0,
		};
	}

	if (num === 0) {
		// Fast path
		return {
			value: 0n,
			decimal: 0,
		};
	}

	const factored = num * factorNum;

	if (factored < Number.MAX_VALUE) {
		return convertToBigInt(factored);
	} else {
		const big = convertToBigInt(num);
		let value = big.value * factorBig;

		let {value: decimalOverflow, decimal} = convertToMultipliedBigInt(
			big.decimal,
			factorNum,
			factorBig,
		);
		value += decimalOverflow;

		return {
			value,
			decimal,
		};
	}
}

function convertToBigInt(num: bigint | number): BigIntWithDecimal {
	if (typeof num === "bigint") {
		return {
			value: num,
			decimal: 0,
		};
	}

	if (Number.isInteger(num)) {
		return {
			value: BigInt(num),
			decimal: 0,
		};
	}

	const int = Math.floor(num);
	const decimal = num - int;
	return {
		value: BigInt(int),
		decimal,
	};
}

function fromDerivedDurations(
	a: Duration,
	b: Duration,
	opts: DurationOptions,
): Duration {
	let {value, decimal, approx} = opts;

	if (decimal !== undefined) {
		// Add together the decimals, if they overflow over 1, then add it to the value
		const {decimal: trueDecimal, value: decimalOverflow} = convertToBigInt(
			decimal,
		);
		decimal = trueDecimal;
		value += decimalOverflow;
	}

	if (approx === undefined) {
		approx = a.approx || b.approx;
	}

	return new Duration({
		approx,
		value,
		decimal,
	});
}

export default class Duration {
	constructor({value, decimal = 0, approx = false}: DurationOptions) {
		this.value = value;
		this.approx = approx;
		this.decimal = decimal;
	}

	public approx: boolean;
	private value: bigint;
	private decimal: number;

	public add(other: Duration, approx?: boolean): Duration {
		return fromDerivedDurations(
			this,
			other,
			{
				value: this.value + other.value,
				decimal: this.decimal + other.decimal,
				approx,
			},
		);
	}

	public subtract(other: Duration, approx?: boolean): Duration {
		return fromDerivedDurations(
			this,
			other,
			{
				value: this.value - other.value,
				decimal: this.decimal - other.decimal,
				approx,
			},
		);
	}

	public divide(num: number, approx?: boolean): Duration {
		return new Duration({
			value: this.value / BigInt(num),
			decimal: this.decimal / num,
			approx,
		});
	}

	public static fromSeconds(
		s: number,
		opts?: DurationFromNumberOptions,
	): Duration {
		return new Duration({
			...opts,
			...convertToMultipliedBigInt(s, 1_000_000_000, 1000000000n),
		});
	}

	public static fromMilliseconds(
		ms: number,
		opts?: DurationFromNumberOptions,
	): Duration {
		return new Duration({
			...opts,
			...convertToMultipliedBigInt(ms, 1_000_000, 1000000n),
		});
	}

	public static fromMicroseconds(
		micro: number | bigint,
		opts?: DurationFromBigIntOptions,
	): Duration {
		return new Duration({
			...opts,
			...convertToMultipliedBigInt(micro, 1_000, 1000n),
		});
	}

	public static fromNanoseconds(
		ns: number | bigint,
		opts?: DurationFromBigIntOptions,
	): Duration {
		return new Duration({
			...opts,
			...convertToBigInt(ns),
		});
	}

	public toSeconds(): number {
		return Number(this.value / 1000000000n);
	}

	public toMilliseconds(): number {
		// Bigint division loses some precision, so get it back
		const int =
			Number(this.value / 1000000n) + Number(this.value % 1000000n) / 1_000_000;
		const decimal = this.decimal / 1_000_000;
		return int + decimal;
	}

	public toNanoseconds(): bigint {
		return this.value;
	}

	public format({longform = false}: FormatOptions = {}): string {
		const ms = this.toMilliseconds();
		if (ms === 0 || ms < 1_000) {
			return formatUnit("milliseconds", 0, longform);
		}

		let parts: string[] = [];
		let left = ms;

		for (const [key, factor] of formatUnitOrder) {
			if (left < factor && key !== "seconds") {
				continue;
			}

			const float = left / factor;
			const value = key === "seconds" ? float : Math.floor(float);

			if (value > 0) {
				parts.push(formatUnit(key, value, longform));
				left -= value * factor;
			}
		}

		return parts.join(longform ? " " : "");
	}

	public setTimeout(callback: () => void): Resource {
		const timer = setTimeout(
			() => {
				callback();
				resource.release();
			},
			this.toMilliseconds(),
		);

		const resource = createTimeoutResource("setTimeout", this, timer);
		return resource;
	}

	public setInterval(callback: () => void): Resource {
		return createTimeoutResource(
			"SetInterval",
			this,
			setInterval(callback, this.toMilliseconds()),
		);
	}
}

type FormatOptions = {
	longform?: boolean;
};

type FormatUnit =
	| "milliseconds"
	| "seconds"
	| "minutes"
	| "hours"
	| "days"
	| "weeks"
	| "months"
	| "years";

const formatUnitSuffixes: {[Key in FormatUnit]: [string, string]} = {
	milliseconds: ["ms", "millisecond"],
	seconds: ["s", "second"],
	minutes: ["m", "minute"],
	hours: ["h", "hour"],
	days: ["d", "day"],
	weeks: ["w", "week"],
	// Same shorthand as minutes but the placement will remove ambiguity
	months: ["m", "month"],
	years: ["y", "year"],
};

const formatUnitOrder: [FormatUnit, number][] = [
	["years", 31_557_600_000],
	["months", 2_629_800_000],
	["weeks", 604_800_000],
	["days", 86_400_000],
	["hours", 3_600_000],
	["minutes", 60_000],
	["seconds", 1_000],
];

function formatUnit(unit: FormatUnit, num: number, longform: boolean): string {
	const fixed = Math.round(num * 100) / 100;
	const str = Number.isInteger(fixed) ? String(fixed) : fixed.toFixed(2);
	const suffixes = formatUnitSuffixes[unit];

	if (longform) {
		let suffix = suffixes[1];
		if (str !== "1") {
			suffix += "s";
		}
		return `${str} ${suffix}`;
	} else {
		return `${str}${suffixes[0]}`;
	}
}

function createTimeoutResource(
	methodName: string,
	duration: Duration,
	timeout: NodeJS.Timeout,
): Resource {
	return createResourceFromCallback(
		`${methodName}<${duration.format()}>`,
		() => {
			clearTimeout(timeout);
		},
		{optional: true},
	);
}

enhanceNodeInspectClass(
	Duration,
	(inst) => {
		let inner = "";
		const s = inst.toSeconds();
		if (Number.isInteger(s)) {
			inner = `${s}s`;
		} else {
			const ms = inst.toMilliseconds();
			if (Number.isInteger(ms)) {
				inner = `${ms}ms`;
			} else {
				inner = `${String(inst.toNanoseconds)}ns`;
			}
		}
		return `Duration<${inner}>`;
	},
);
