export type IntSize = 0 | 1 | 2 | 4 | 8;

export function writeInt(
	value: bigint | number,
	size: IntSize,
	offset: number,
	view: DataView,
) {
	if (typeof value === "bigint") {
		if (size === 8) {
			view.setBigInt64(offset, value);
			return;
		} else {
			throw new Error(`Expected size 8 for bigint but got ${size}`);
		}
	}

	switch (size) {
		case 1: {
			view.setInt8(offset, value);
			break;
		}

		case 2: {
			view.setInt16(offset, value);
			break;
		}

		case 4: {
			view.setInt32(offset, value);
			break;
		}

		default:
			throw new Error(`Unsupported integer size ${size}`);
	}
}
