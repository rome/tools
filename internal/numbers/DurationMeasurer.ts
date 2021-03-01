import Duration from "./Duration";

// Get the most accurate high resolution timestamp available
function now(): bigint {
	if (typeof process === "undefined") {
		return BigInt(performance.now() * 1_000);
	} else {
		return process.hrtime.bigint();
	}
}

export default class DurationMeasurer {
	constructor() {
		this.start = now();
	}

	private start: bigint;

	public since(): Duration {
		return Duration.fromNanoseconds(now() - this.start);
	}
}
