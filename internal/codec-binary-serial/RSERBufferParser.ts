import {
	AnyRSERFilePathMap,
	IntSize,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
} from "./types";
import {
	FILE_CODES,
	VALUE_CODES,
	errorCodeToConstructor,
	filePathFromCode,
	filePathMapFromCode,
	filePathSetFromCode,
	formatCode,
	validateErrorCode,
	validateFileCode,
	validateValueCode,
} from "./codes";
import {AnyFilePath, AnyFilePathSet} from "@internal/path";
import {Class} from "@internal/typescript-helpers";
import {
	ErrorFrames,
	StructuredNodeSystemErrorProperties,
	setErrorFrames,
	setNodeErrorProps,
} from "@internal/v8";

const textDecoder = new TextDecoder();

export default class RSERBufferParser {
	constructor(view: DataView) {
		this.view = view;
		this.readOffset = 0;
	}

	view: DataView;
	readOffset: number;

	getReadableSize(): number {
		return this.view.byteLength - this.readOffset;
	}

	canRead(size: number): boolean {
		return this.getReadableSize() >= size;
	}

	assertReadableSize(size: number) {
		let remaining = this.getReadableSize();
		if (remaining < size) {
			throw new Error(
				`Expected at least ${size} bytes to read but only have ${remaining}`,
			);
		}
	}

	peekString(size: number): string {
		this.assertReadableSize(size);
		return textDecoder.decode(
			new DataView(this.view.buffer, this.readOffset, size),
		);
	}

	readString(): string {
		const size = this.decodeNumber();
		const str = this.peekString(size);
		this.readOffset += size;
		return str;
	}

	peekInt(size: 1): number
	peekInt(size: 2): number
	peekInt(size: 4): number
	peekInt(size: 8): bigint
	peekInt(size: IntSize): number | bigint
	peekInt(size: IntSize): number | bigint {
		this.assertReadableSize(size);

		switch (size) {
			case 1:
				return this.view.getInt8(this.readOffset);

			case 2:
				return this.view.getInt16(this.readOffset);

			case 4:
				return this.view.getInt32(this.readOffset);

			case 8:
				return this.view.getBigInt64(this.readOffset);

			default:
				throw new Error(`Invalid integer size ${size}`);
		}
	}

	peekCode(): VALUE_CODES {
		return validateValueCode(this.peekInt(1));
	}

	readInt(bytes: 1): number
	readInt(bytes: 2): number
	readInt(bytes: 4): number
	readInt(bytes: 8): bigint
	readInt(bytes: IntSize): number | bigint
	readInt(bytes: IntSize): number | bigint {
		const ival = this.peekInt(bytes);
		this.readOffset += bytes;
		return ival;
	}

	expectCode(expected: number): void {
		const got = this.peekCode();
		if (got === expected) {
			this.readOffset++;
		} else {
			throw new Error(
				`Expected code ${formatCode(expected)} but got ${formatCode(got)}`,
			);
		}
	}

	decodeHeader(): false | number {
		this.expectCode(0);
		this.expectCode(1);

		if (this.canRead(1)) {
			const size = this.getDecodeIntSize();
			if (this.canRead(1 + size)) {
				return this.decodeNumber();
			}
		}

		return false;
	}

	decodeSymbol(): symbol {
		this.readOffset++;
		const key = this.readString();
		return Symbol.for(key);
	}

	decodeTrue(): true {
		this.readOffset++;
		return true;
	}

	decodeFalse(): false {
		this.readOffset++;
		return false;
	}

	decodeNull(): null {
		this.readOffset++;
		return null;
	}

	decodeNaN(): number {
		this.readOffset++;
		return NaN;
	}

	decodePositiveInfinity(): number {
		this.readOffset++;
		return Number.POSITIVE_INFINITY;
	}

	decodeNegativeInfinity(): number {
		this.readOffset++;
		return Number.NEGATIVE_INFINITY;
	}

	decodeNegativeZero(): number {
		this.readOffset++;
		return -0;
	}

	decodeUndefined(): undefined {
		this.readOffset++;
		return undefined;
	}

	decodeFloat(): number {
		this.readOffset++;
		this.assertReadableSize(8);
		const num = this.view.getFloat64(this.readOffset);
		this.readOffset += 8;
		return num;
	}

	decodeValue(): RSERValue {
		const code = this.peekCode();

		switch (code) {
			case VALUE_CODES.INT8:
			case VALUE_CODES.INT16:
			case VALUE_CODES.INT32:
			case VALUE_CODES.INT64:
				return this.decodeInt();

			case VALUE_CODES.FLOAT:
				return this.decodeFloat();

			case VALUE_CODES.SYMBOL:
				return this.decodeSymbol();

			case VALUE_CODES.TRUE:
				return this.decodeTrue();

			case VALUE_CODES.FALSE:
				return this.decodeFalse();

			case VALUE_CODES.NULL:
				return this.decodeNull();

			case VALUE_CODES.UNDEFINED:
				return this.decodeUndefined();

			case VALUE_CODES.NAN:
				return this.decodeNaN();

			case VALUE_CODES.POSITIVE_INFINITY:
				return this.decodePositiveInfinity();

			case VALUE_CODES.NEGATIVE_INFINITY:
				return this.decodeNegativeInfinity();

			case VALUE_CODES.NEGATIVE_ZERO:
				return this.decodeNegativeZero();

			case VALUE_CODES.FILE_PATH:
				return this.decodeFilePath();

			case VALUE_CODES.FILE_PATH_MAP:
				return this.decodeFilePathMap();

			case VALUE_CODES.FILE_PATH_SET:
				return this.decodeFilePathSet();

			case VALUE_CODES.SET:
				return this.decodeSet();

			case VALUE_CODES.MAP:
				return this.decodeMap();

			case VALUE_CODES.ERROR:
				return this.decodeError();

			case VALUE_CODES.STRING:
				return this.decodeString();

			case VALUE_CODES.ARRAY:
				return this.decodeArray();

			case VALUE_CODES.OBJECT:
				return this.decodeObject();

			case VALUE_CODES.REGEXP:
				return this.decodeRegExp();

			case VALUE_CODES.TEMPLATED_OBJECT_ARRAY:
				return this.decodeTemplatedObjectArray();

			case VALUE_CODES.DATE:
				return this.decodeDate();

			default:
				throw new Error(`Unhandled ${formatCode(code)} code`);
		}
	}

	decodeRegExp(): RegExp {
		this.expectCode(VALUE_CODES.REGEXP);
		const pattern = this.readString();
		const flags = this.readString();
		return new RegExp(pattern, flags);
	}

	decodeArray(): RSERArray {
		this.expectCode(VALUE_CODES.ARRAY);
		const nitems = this.decodeInt();
		const arr: RSERArray = [];
		for (let i = 0; i < nitems; ++i) {
			arr.push(this.decodeValue());
		}
		return arr;
	}

	decodeObject(): RSERObject {
		this.expectCode(VALUE_CODES.OBJECT);
		const nitems = this.decodeInt();
		const res: RSERObject = {};
		for (let i = 0; i < nitems; ++i) {
			const key = this.decodeString();
			const val = this.decodeValue();
			res[key] = val;
		}
		return res;
	}

	decodeTemplatedObjectArray(): RSERArray {
		// Sometimes we may encode a templated object array to a regular array (like when there's no elements)
		const code = this.peekCode();
		if (code === VALUE_CODES.ARRAY) {
			return this.decodeArray();
		}

		this.expectCode(VALUE_CODES.TEMPLATED_OBJECT_ARRAY);

		// Decode keys
		const nkeys = this.decodeInt();
		const keys: Array<string> = [];
		for (let i = 0; i < nkeys; ++i) {
			keys.push(this.readString());
		}

		// Decode array and objects
		const nitems = this.decodeInt();
		const arr: RSERArray = [];
		for (let i = 0; i < nitems; ++i) {
			const obj: RSERObject = {};
			for (let keyidx = 0; keyidx < keys.length; ++keyidx) {
				const val = this.decodeValue();
				const key = keys[keyidx];
				obj[key] = val;
			}
			arr.push(obj);
		}
		return arr;
	}

	decodeDate(): Date {
		this.expectCode(VALUE_CODES.DATE);
		const time = this.decodeNumber();
		return new Date(time);
	}

	decodeFilePathCode(): FILE_CODES {
		return validateFileCode(this.readInt(1));
	}

	decodeFilePath(): AnyFilePath {
		this.expectCode(VALUE_CODES.FILE_PATH);
		const code = this.decodeFilePathCode();
		const str = this.readString();
		return filePathFromCode(code, str);
	}

	decodeFilePathMap(): AnyRSERFilePathMap {
		this.expectCode(VALUE_CODES.FILE_PATH_MAP);

		const code = this.decodeFilePathCode();
		const map: AnyRSERFilePathMap = filePathMapFromCode(code);

		const nitems = this.decodeInt();
		for (let i = 0; i < nitems; ++i) {
			const str = this.readString();
			const value = this.decodeValue();
			map.setString(str, value);
		}
		return map;
	}

	decodeFilePathSet(): AnyFilePathSet {
		this.expectCode(VALUE_CODES.FILE_PATH_SET);

		const code = this.decodeFilePathCode();
		const set = filePathSetFromCode(code);

		const nitems = this.decodeInt();
		for (let i = 0; i < nitems; ++i) {
			set.addString(this.readString());
		}
		return set;
	}

	decodeSet(): RSERSet {
		this.expectCode(VALUE_CODES.SET);
		const nitems = this.decodeInt();
		const set: RSERSet = new Set();
		for (let i = 0; i < nitems; ++i) {
			set.add(this.decodeValue());
		}
		return set;
	}

	decodeMap(): RSERMap {
		this.expectCode(VALUE_CODES.MAP);
		const map: RSERMap = new Map();
		const nitems = this.decodeInt();
		for (let i = 0; i < nitems; ++i) {
			const key = this.decodeValue();
			const value = this.decodeValue();
			map.set(key, value);
		}
		return map;
	}

	decodeError(): Error {
		this.readOffset++;

		const errorCode = validateErrorCode(this.readInt(1));
		const message = this.decodeStringOrVoid();
		const stack = this.decodeStringOrVoid();

		const ErrorConstructor: Class<Error> = errorCodeToConstructor(errorCode);
		const err = new ErrorConstructor(message);
		err.stack = stack;

		// @ts-ignore: Validating these is expensive but we can be confident on the validity
		const nodeProps: StructuredNodeSystemErrorProperties = this.decodeObject();
		setNodeErrorProps(err, nodeProps);

		// @ts-ignore: ^^
		const frames: ErrorFrames = this.decodeTemplatedObjectArray();
		setErrorFrames(err, frames);

		return err;
	}

	decodeStringOrVoid(): string | undefined {
		const code = this.peekCode();
		if (code === VALUE_CODES.UNDEFINED) {
			return this.decodeUndefined();
		} else if (code === VALUE_CODES.STRING) {
			return this.decodeString();
		} else {
			throw new Error(
				`Expected string or undefined but got ${formatCode(code)}`,
			);
		}
	}

	decodeNumberOrVoid(): bigint | number | undefined {
		const code = this.peekCode();

		switch (code) {
			case VALUE_CODES.UNDEFINED:
				return this.decodeUndefined();

			case VALUE_CODES.INT8:
			case VALUE_CODES.INT16:
			case VALUE_CODES.INT32:
			case VALUE_CODES.INT64:
				return this.decodeInt();

			default:
				throw new Error(
					`Expected number or undefined but got ${formatCode(code)}`,
				);
		}
	}

	decodeString(): string {
		this.expectCode(VALUE_CODES.STRING);
		return this.readString();
	}

	decodeInt(): bigint | number {
		this.assertReadableSize(1);
		const size = this.getDecodeIntSize();
		this.readOffset += 1;
		return this.readInt(size);
	}

	decodeNumber(): number {
		const code = this.peekCode();

		switch (code) {
			case VALUE_CODES.INT8:
			case VALUE_CODES.INT16:
			case VALUE_CODES.INT32: {
				const num = this.decodeInt();
				if (typeof num === "bigint") {
					throw new Error("Did not expect a bigint");
				} else {
					return num;
				}
			}

			case VALUE_CODES.FLOAT:
				return this.decodeFloat();

			case VALUE_CODES.INT64:
				throw new Error("Unexpected bigint, only regular numbers accepted");

			default:
				throw new Error(`${formatCode(code)} is not a valid number code`);
		}
	}

	getDecodeIntSize(): IntSize {
		const code = this.peekInt(1);
		switch (code) {
			case VALUE_CODES.INT8:
				return 1;

			case VALUE_CODES.INT16:
				return 2;

			case VALUE_CODES.INT32:
				return 4;

			case VALUE_CODES.INT64:
				return 8;

			default:
				throw new Error(`No int encoding for ${formatCode(code)}`);
		}
	}
}
