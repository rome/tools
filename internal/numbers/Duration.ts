import { enhanceNodeInspectClass } from "@internal/node";

type DurationHumanizeOptions = {
  allowMilliseconds?: boolean;
  secondFractionDigits?: number;
  longform?: boolean;
};

const shortSuffixes = {
	ms: "ms",
	s: "s",
	m: "m",
	h: "h",
};

const longSuffixes = {
	ms: "millisecond",
	s: "second",
	m: "minute",
	h: "hour",
};

function format(
	key: keyof typeof shortSuffixes,
	num: string | number,
	longform: boolean,
): string {
	const str = String(num);
	if (longform) {
		let suffix = longSuffixes[key];
		if (str !== "1") {
			suffix += "s";
		}
		return `${str} ${suffix} `;
	} else {
		return `${str}${shortSuffixes[key]}`;
	}
}

type DurationOptions = {
  approx?: boolean;
  decimal?: number;
  value: bigint;
};

type DurationFromBigIntOptions = Omit<DurationOptions, "value">;
type DurationFromNumberOptions = Omit<DurationOptions, "value" | "precision">;

type BigIntWithDecimal = {value: bigint, decimal: number};

function convertToMultipliedBigInt(num: bigint | number, factorNum: number, factorBig: bigint): BigIntWithDecimal {
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
    
    let {value: decimalOverflow, decimal} = convertToMultipliedBigInt(big.decimal, factorNum, factorBig);
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

function fromDerivedDurations(a: Duration, b: Duration, opts: DurationOptions): Duration {
  let {value, decimal, approx} = opts;

  if (decimal !== undefined) {
    // Add together the decimals, if they overflow over 1, then add it to the value
    const {decimal: trueDecimal, value: decimalOverflow} = convertToBigInt(decimal);
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
    return fromDerivedDurations(this, other, {
      value: this.value + other.value,
      decimal: this.decimal + other.decimal,
      approx,
    });
  }

  public subtract(other: Duration, approx?: boolean): Duration {
    return fromDerivedDurations(this, other, {
      value: this.value - other.value,
      decimal: this.decimal - other.decimal,
      approx,
    });
  }

  public divide(num: number, approx?: boolean): Duration {
    return new Duration({
      value: this.value / BigInt(num),
      decimal: this.decimal / num,
      approx,
    });
  }

  public static fromSeconds(s: number, opts?: DurationFromNumberOptions): Duration {
    return new Duration({
      ...opts,
      ...convertToMultipliedBigInt(s, 1000000000, 1000000000n),
    });
  }

  public static fromMilliseconds(ms: number, opts?: DurationFromNumberOptions): Duration {
    return new Duration({
      ...opts,
      ...convertToMultipliedBigInt(ms, 1000000, 1000000n),
    });
  }

  public static fromMicroseconds(micro: number | bigint, opts?: DurationFromBigIntOptions): Duration {
    return new Duration({
      ...opts,
      ...convertToMultipliedBigInt(micro, 1000, 1000n),
    });
  }

  public static fromNanoseconds(ns: number | bigint, opts?: DurationFromBigIntOptions): Duration {
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
    const int = Number(this.value / 1000000n) + (Number(this.value % 1000000n) / 1000000);
    const decimal = this.decimal / 1000000;
    return int + decimal;
  }

  public toNanoseconds(): bigint {
    return this.value;
  }

  public format(
    {allowMilliseconds = false, longform = false, secondFractionDigits = 2}: DurationHumanizeOptions = {},
  ): string {
    const ms = this.toMilliseconds();

    if (ms === 0) {
      if (allowMilliseconds) {
        return format("ms", 0, longform);
      } else {
        return format("s", 0, longform);
      }
    }
  
    const s = Math.floor(ms / 1_000);
    const m = Math.floor(s / 60);
    const h = Math.floor(m / 60);
  
    if (h === 0 && m === 0 && s === 0) {
      if (allowMilliseconds) {
        return format("ms", ms, longform);
      } else {
        return format("s", (ms / 1_000).toFixed(secondFractionDigits), longform);
      }
    }
  
    let buf = "";
  
    if (h > 0) {
      buf += format("h", h, longform);
    }
  
    if (m > 0) {
      buf += format("m", m % 60, longform);
    }
  
    if (allowMilliseconds) {
      buf += format(
        "s",
        (ms / 1_000 % 60).toFixed(secondFractionDigits),
        longform,
      );
    } else {
      buf += format("s", s % 60, longform);
    }
  
    if (longform) {
      buf = buf.trimRight();
    }
  
    return buf;
  }

  public setTimeout(callback: () => void): NodeJS.Timeout {
    return setTimeout(callback, this.toMilliseconds());
  }

  public setInterval(callback: () => void): NodeJS.Timeout {
    return setInterval(callback, this.toMilliseconds());
  }
}

enhanceNodeInspectClass(Duration, (inst) => {
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
})