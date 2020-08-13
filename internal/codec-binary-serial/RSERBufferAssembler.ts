import {
	VALUE_CODES,
	filePathMapToCode,
	filePathSetToCode,
	filePathToCode,
	instanceToErrorCode,
} from "./codes";
import {
	AnyRSERFilePathMap,
	IntSize,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
} from "./types";
import {UnionToIntersection, isPlainObject} from "@internal/typescript-helpers";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyFilePath,
	AnyFilePathSet,
	RelativeFilePath,
	RelativeFilePathMap,
	RelativeFilePathSet,
	URLFilePath,
	UnknownFilePath,
	UnknownFilePathMap,
	UnknownFilePathSet,
} from "@internal/path";
import {getErrorStructure} from "@internal/v8";
import {pretty} from "@internal/pretty-format";

const MAX_INT8 = 127;
const MAX_INT16 = 32_767;
const MAX_INT32 = 2_147_483_647;

export default class RSERBufferAssembler {
	constructor() {
		this.totalSize = 0;
	}

	static measure(
		val: RSERValue,
	): {
		payloadLength: number;
		messageLength: number;
	} {
		const observer = new RSERBufferAssembler();
		observer.encodeValue(val);
		const payloadLength = observer.totalSize;
		observer.encodeHeader(payloadLength);
		const messageLength = observer.totalSize;
		return {payloadLength, messageLength};
	}

	totalSize: number;

	writeCode(code: number) {
		this.totalSize += 1;
		code;
	}

	writeByte(value: number) {
		this.totalSize += 1;
		value;
	}

	writeInt(value: bigint | number, size: IntSize) {
		this.totalSize += size;
	}

	writeFloat(value: number) {
		value;
		this.totalSize += 8;
	}

	appendString(buf: string) {
		this.totalSize += Buffer.byteLength(buf);
	}

	appendArray(buf: Uint8Array, offset: number) {
		this.totalSize += buf.byteLength - offset;
	}

	encodeHeader(size: number) {
		this.writeByte(0);
		this.writeByte(1);
		this.encodeInt(size);
	}

	encodeBigInt(val: bigint) {
		this.writeCode(VALUE_CODES.INT64);
		this.writeInt(val, 8);
	}

	encodeInt(val: bigint | number) {
		if (typeof val === "bigint") {
			return this.encodeBigInt(val);
		}

		const abs = Math.abs(val);
		if (abs <= MAX_INT8) {
			this.writeCode(VALUE_CODES.INT8);
			this.writeInt(val, 1);
		} else if (abs <= MAX_INT16) {
			this.writeCode(VALUE_CODES.INT16);
			this.writeInt(val, 2);
		} else if (abs <= MAX_INT32) {
			this.writeCode(VALUE_CODES.INT32);
			this.writeInt(val, 4);
		} else {
			this.encodeFloat(val);
		}
	}

	encodeTemplatedObjectArray<Value extends RSERObject &
		UnionToIntersection<Value>>(arr: Array<Value>) {
		if (arr.length === 0) {
			this.writeCode(VALUE_CODES.ARRAY);
			this.encodeInt(0);
			return;
		}

		this.writeCode(VALUE_CODES.TEMPLATED_OBJECT_ARRAY);

		// Encode keys
		const keys: Array<string> = Object.keys(arr[0]);
		this.encodeInt(keys.length);
		for (const key of keys) {
			this.encodeStringValue(key);
		}

		// Encode entries
		this.encodeInt(arr.length);
		for (const obj of arr) {
			for (const key of keys) {
				const val = obj[key];
				this.encodeValue(val);
			}
		}
	}

	encodeArray(val: RSERArray) {
		this.writeCode(VALUE_CODES.ARRAY);
		this.encodeInt(val.length);
		for (let i = 0; i < val.length; ++i) {
			this.encodeValue(val[i]);
		}
	}

	encodeSet(set: RSERSet) {
		this.writeCode(VALUE_CODES.SET);
		this.encodeInt(set.size);
		for (const elem of set) {
			this.encodeValue(elem);
		}
	}

	encodeMap(map: RSERMap) {
		this.writeCode(VALUE_CODES.MAP);
		this.encodeInt(map.size);
		for (const [key, value] of map) {
			this.encodeValue(key);
			this.encodeValue(value);
		}
	}

	encodeFilePathMap(map: AnyRSERFilePathMap) {
		this.writeCode(VALUE_CODES.FILE_PATH_MAP);
		this.writeByte(filePathMapToCode(map));
		this.encodeInt(map.size);
		for (const [path, value] of map) {
			this.encodeStringValue(path.join());
			this.encodeValue(value);
		}
	}

	encodeFilePathSet(set: AnyFilePathSet) {
		this.writeCode(VALUE_CODES.FILE_PATH_SET);
		this.writeByte(filePathSetToCode(set));
		this.encodeInt(set.size);
		for (const path of set) {
			this.encodeStringValue(path.join());
		}
	}

	encodeFilePath(path: AnyFilePath) {
		this.writeCode(VALUE_CODES.FILE_PATH);
		this.writeByte(filePathToCode(path));
		this.encodeStringValue(path.join());
	}

	encodeDate(val: Date) {
		this.writeCode(VALUE_CODES.DATE);
		this.encodeInt(val.valueOf());
	}

	encodeError(val: Error) {
		this.writeCode(VALUE_CODES.ERROR);
		this.writeCode(instanceToErrorCode(val));

		const struct = getErrorStructure(val, 0, false);
		this.encodeValue(struct.message);
		this.encodeValue(struct.stack);
		this.encodePlainObject(struct.node);
		this.encodeTemplatedObjectArray(struct.frames);

		throw new Error("TODO");
	}

	encodeNull() {
		this.writeCode(VALUE_CODES.NULL);
	}

	encodeRegExp(regex: RegExp) {
		this.writeCode(VALUE_CODES.REGEXP);
		this.encodeStringValue(regex.source);
		this.encodeStringValue(regex.flags);
	}

	encodeObject(val: Extract<RSERValue, object>) {
		if (
			val instanceof UnknownFilePath ||
			val instanceof RelativeFilePath ||
			val instanceof AbsoluteFilePath ||
			val instanceof URLFilePath
		) {
			return this.encodeFilePath(val);
		}

		if (val instanceof Set) {
			return this.encodeSet(val);
		}

		if (val instanceof Map) {
			return this.encodeMap(val);
		}

		if (val instanceof Error) {
			return this.encodeError(val);
		}

		if (val instanceof RegExp) {
			return this.encodeRegExp(val);
		}

		if (
			val instanceof RelativeFilePathMap ||
			val instanceof AbsoluteFilePathMap ||
			val instanceof UnknownFilePathMap
		) {
			return this.encodeFilePathMap(val);
		}

		if (
			val instanceof RelativeFilePathSet ||
			val instanceof AbsoluteFilePathSet ||
			val instanceof UnknownFilePathSet
		) {
			return this.encodeFilePathSet(val);
		}

		if (Array.isArray(val)) {
			return this.encodeArray(val);
		}

		if (val instanceof Date) {
			return this.encodeDate(val);
		}

		if (isPlainObject(val)) {
			this.encodePlainObject(val);
		} else {
			throw new Error(
				pretty`Don't know how to serialize the object ${val} to RSER`,
			);
		}
	}

	encodePlainObject(val: RSERObject) {
		this.writeCode(VALUE_CODES.OBJECT);

		const keys = Object.keys(val);

		// First pass to compute number of defined keys
		let numKeys = keys.length;
		for (let i = 0; i < keys.length; ++i) {
			const key = keys[i];
			const v = val[key];
			if (typeof v === "undefined") {
				numKeys--;
			}
		}

		this.encodeInt(numKeys);

		for (let i = 0; i < keys.length; ++i) {
			const key = keys[i];
			const v = val[key];
			if (typeof v === "undefined") {
				// Don't include it
				continue;
			}

			this.encodeValue(key);
			this.encodeValue(v);
		}
	}

	encodeUndefined() {
		this.writeCode(VALUE_CODES.UNDEFINED);
	}

	encodeStringValue(val: string) {
		this.encodeInt(Buffer.byteLength(val));
		this.appendString(val);
	}

	encodeNumber(val: bigint | number) {
		if (typeof val === "bigint" || (isFinite(val) && Math.floor(val) === val)) {
			this.encodeInt(val);
		} else {
			this.encodeFloat(val);
		}
	}

	encodeFloat(val: number) {
		this.writeCode(VALUE_CODES.FLOAT);
		this.writeFloat(val);
	}

	encodeValue(val: RSERValue) {
		switch (typeof val) {
			case "bigint":
			case "number": {
				// NaN
				if (typeof val === "number" && isNaN(val)) {
					this.writeCode(VALUE_CODES.NAN);
					return;
				}

				// -0
				if (Object.is(val, -0)) {
					this.writeCode(VALUE_CODES.NEGATIVE_ZERO);
					return;
				}

				// +Infinity
				if (val === Number.POSITIVE_INFINITY) {
					this.writeCode(VALUE_CODES.POSITIVE_INFINITY);
					return;
				}

				// -Infinity
				if (val === Number.NEGATIVE_INFINITY) {
					this.writeCode(VALUE_CODES.NEGATIVE_INFINITY);
					return;
				}

				this.encodeNumber(val);
				return;
			}

			case "undefined": {
				return this.encodeUndefined();
			}

			case "string": {
				this.writeCode(VALUE_CODES.STRING);
				this.encodeStringValue(val);
				return;
			}

			case "boolean": {
				this.writeByte(val ? VALUE_CODES.TRUE : VALUE_CODES.FALSE);
				return;
			}

			case "symbol": {
				this.writeCode(VALUE_CODES.SYMBOL);
				const key = Symbol.keyFor(val);
				if (key === undefined) {
					throw new Error("Not a global symbol");
				}
				this.encodeStringValue(key);
				return;
			}

			case "object": {
				if (val === null) {
					return this.encodeNull();
				}

				return this.encodeObject(val);
			}
		}

		throw new Error(
			pretty`Don't know how to serialize the value ${val} to RSER`,
		);
	}
}
