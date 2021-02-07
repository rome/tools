import {
	AnyRSERFilePathMap,
	IntSize,
	RSERArray,
	RSERArrayBufferView,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
} from "./types";
import {
	FILE_CODES,
	VALUE_CODES,
	VERSION,
	arrayBufferViewCodeToInstance,
	errorCodeToInstance,
	filePathFromCode,
	filePathMapFromCode,
	filePathSetFromCode,
	formatCode,
	validateArrayBufferViewCode,
	validateErrorCode,
	validateFileCode,
	validateValueCode,
} from "./constants";
import {AnyFilePath, AnyFilePathSet} from "@internal/path";
import {
	ErrorFrames,
	StructuredNodeSystemErrorProperties,
	setErrorFrames,
	setNodeErrorProps,
} from "@internal/v8";
import {utf8Decode} from "./utf8";
import {CachedKeyDecoder} from "@internal/codec-binary-serial/CachedKeyDecoder";
import {ExtendedMap} from "@internal/collections";
import RSERParserError from "./RSERParserError";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {Position, SourceLocation} from "@internal/parser-core";

const sharedCachedKeyDecoder = new CachedKeyDecoder();

export default class RSERBufferParser {
	constructor(view: DataView) {
		this.view = view;
		this.bytes = new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
		this.readOffset = 0;
		this.references = new ExtendedMap("references");
	}

	private references: ExtendedMap<number, RSERValue>;
	private view: DataView;
	private bytes: Uint8Array;
	public readOffset: number;

	public getReadableSize(): number {
		return this.view.byteLength - this.readOffset;
	}

	private canRead(size: number): boolean {
		return this.getReadableSize() >= size;
	}

	private assertReadableSize(size: number) {
		let remaining = this.getReadableSize();

		if (remaining < size) {
			throw this.unexpected(
				`Expected at least ${size} bytes to read but only have ${remaining}`,
			);
		}
	}

	private peekString(size: number): string {
		this.assertReadableSize(size);
		return utf8Decode(this.bytes, this.readOffset, size);
	}

	private readStringSize(size: number): string {
		const str = this.peekString(size);
		this.readOffset += size;
		return str;
	}

	private readString(): string {
		const size = this.decodeNumber();
		if (size === 0) {
			return "";
		} else {
			return this.readStringSize(size);
		}
	}

	private peekInt(size: 1, offset?: number): number
	private peekInt(size: 2, offset?: number): number
	private peekInt(size: 4, offset?: number): number
	private peekInt(size: 8, offset?: number): bigint
	private peekInt(size: IntSize, offset?: number): number | bigint
	private peekInt(size: IntSize, offset: number = 0): number | bigint {
		this.assertReadableSize(size);

		switch (size) {
			case 1:
				return this.view.getInt8(this.readOffset + offset);

			case 2:
				return this.view.getInt16(this.readOffset + offset);

			case 4:
				return this.view.getInt32(this.readOffset + offset);

			case 8:
				return this.view.getBigInt64(this.readOffset + offset);

			default:
				throw this.unexpected(`Invalid integer size ${size}`);
		}
	}

	private peekCode(): VALUE_CODES {
		return validateValueCode(this.peekInt(1));
	}

	private readInt(bytes: 1): number
	private readInt(bytes: 2): number
	private readInt(bytes: 4): number
	private readInt(bytes: 8): bigint
	private readInt(bytes: IntSize): number | bigint
	private readInt(bytes: IntSize): number | bigint {
		const ival = this.peekInt(bytes);
		this.readOffset += bytes;
		return ival;
	}

	private unexpected(message: string, offset: number = this.readOffset) {
		throw new RSERParserError(`${message} at offset ${offset}`);
	}

	private expectCode(expected: number): void {
		const got = this.peekCode();
		if (got === expected) {
			this.readOffset++;
		} else {
			this.unexpected(
				`Expected code ${formatCode(expected)} but got ${formatCode(got)}`,
			);
		}
	}

	public getUnreadBuffer(): Uint8Array {
		return this.bytes.slice(this.readOffset);
	}

	public maybeDecodeStreamHeader(): "INCOMPLETE" | "INCOMPATIBLE" | "VALID" {
		const prevReadOffset = this.readOffset;

		if (this.canRead(1)) {
			const got = this.peekCode();
			if (got === VALUE_CODES.STREAM_HEADER) {
				this.expectCode(VALUE_CODES.STREAM_HEADER);
			} else {
				return "INCOMPATIBLE";
			}
		} else {
			this.readOffset = prevReadOffset;
			return "INCOMPLETE";
		}

		const version = this.maybeDecodeNumber();
		if (version === undefined) {
			this.readOffset = prevReadOffset;
			return "INCOMPLETE";
		}

		if (version !== VERSION) {
			return "INCOMPATIBLE";
		}

		return "VALID";
	}

	public maybeDecodeMessageHeader(): false | number {
		if (this.canRead(1)) {
			this.expectCode(VALUE_CODES.MESSAGE_HEADER);
		} else {
			return false;
		}

		const num = this.maybeDecodeNumber();
		if (num !== undefined) {
			return num;
		}

		return false;
	}

	private maybeDecodeNumber(): undefined | number {
		if (this.canRead(1)) {
			const size = this.getDecodeNumberSize();
			if (size === 0 || this.canRead(1 + size)) {
				return this.decodeNumber();
			}
		}
		return undefined;
	}

	public decodeDeclareReference(): RSERValue {
		this.expectCode(VALUE_CODES.DECLARE_REFERENCE);
		const id = this.decodeNumber();
		const code = this.peekCode();

		switch (code) {
			case VALUE_CODES.FILE_PATH_MAP: {
				this.readOffset++;
				const code = this.decodeFilePathCode();
				const map = filePathMapFromCode(code);
				this.references.set(id, map);
				return this.decodeFilePathMapValue(map);
			}

			case VALUE_CODES.SET: {
				this.readOffset++;
				const set: RSERSet = new Set();
				this.references.set(id, set);
				return this.decodeSetValue(set);
			}

			case VALUE_CODES.MAP: {
				this.readOffset++;
				const map: RSERMap = new Map();
				this.references.set(id, map);
				return this.decodeMapValue(map);
			}

			case VALUE_CODES.ARRAY: {
				const arr: RSERArray = this.decodeArrayHead();
				this.references.set(id, arr);
				return this.decodeArrayElements(arr);
			}

			case VALUE_CODES.TEMPLATED_OBJECT_ARRAY: {
				const arr: RSERArray = this.decodeTemplatedObjectArrayHead();
				this.references.set(id, arr);
				return this.decodeTemplateObjectArrayValues(arr);
			}

			case VALUE_CODES.OBJECT: {
				this.readOffset++;
				const obj: RSERObject = {};
				this.references.set(id, obj);
				return this.decodeObjectValue(obj);
			}

			default: {
				const val = this.decodeNonReferentialValue(code);
				this.references.set(id, val);
				return val;
			}
		}
	}

	public decodeReference(): RSERValue {
		this.expectCode(VALUE_CODES.REFERENCE);
		const id = this.decodeNumber();
		return this.references.assert(id);
	}

	public decodeValue(): RSERValue {
		const code = this.peekCode();

		const ref = this.decodeReferentialValue(code);
		if (ref !== undefined) {
			return ref;
		}

		return this.decodeNonReferentialValue(code);
	}

	// These are values that can hold other values
	private decodeReferentialValue(code: VALUE_CODES): undefined | RSERValue {
		switch (code) {
			case VALUE_CODES.FILE_PATH_MAP:
				return this.decodeFilePathMap();

			case VALUE_CODES.SET:
				return this.decodeSet();

			case VALUE_CODES.MAP:
				return this.decodeMap();

			case VALUE_CODES.ARRAY:
				return this.decodeArray();

			case VALUE_CODES.OBJECT:
				return this.decodeObject();

			case VALUE_CODES.TEMPLATED_OBJECT_ARRAY:
				return this.decodeTemplatedObjectArray();

			case VALUE_CODES.REFERENCE:
				return this.decodeReference();

			case VALUE_CODES.DECLARE_REFERENCE:
				return this.decodeDeclareReference();

			default:
				return undefined;
		}
	}

	private decodeNonReferentialValue(code: VALUE_CODES): RSERValue {
		switch (code) {
			case VALUE_CODES.INT8:
			case VALUE_CODES.INT16:
			case VALUE_CODES.INT32:
			case VALUE_CODES.FLOAT:
			case VALUE_CODES.NEGATIVE_ONE:
			case VALUE_CODES.POSITIVE_ZERO:
			case VALUE_CODES.POSITIVE_ONE:
			case VALUE_CODES.POSITIVE_INFINITY:
			case VALUE_CODES.NEGATIVE_INFINITY:
			case VALUE_CODES.NEGATIVE_ZERO:
				return this.decodeNumber();

			case VALUE_CODES.INT64:
				return this.decodeInt();

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

			case VALUE_CODES.FILE_PATH:
				return this.decodeFilePath();

			case VALUE_CODES.FILE_PATH_SET:
				return this.decodeFilePathSet();

			case VALUE_CODES.ERROR:
				return this.decodeError();

			case VALUE_CODES.STRING:
				return this.decodeString();

			case VALUE_CODES.REGEXP:
				return this.decodeRegExp();

			case VALUE_CODES.DATE:
				return this.decodeDate();

			case VALUE_CODES.ARRAY_BUFFER_VIEW:
				return this.decodeArrayBufferView();

			case VALUE_CODES.ARRAY_BUFFER:
				return this.decodeArrayBuffer();

			case VALUE_CODES.POSITION:
				return this.decodePosition();

			case VALUE_CODES.SOURCE_LOCATION:
				return this.decodeSourceLocation();

			default:
				throw this.unexpected(`Unhandled ${formatCode(code)} code`);
		}
	}

	private decodePosition(): Position {
		this.expectCode(VALUE_CODES.POSITION);
		return this.decodePositionValue();
	}

	private decodePositionValue(): Position {
		return {
			line: ob1Coerce1(this.decodeNumber()),
			column: ob1Coerce0(this.decodeNumber()),
		};
	}

	private decodeSourceLocation(): SourceLocation {
		this.expectCode(VALUE_CODES.SOURCE_LOCATION);
		return {
			filename: this.decodeStringOrVoid(),
			identifierName: this.decodeStringOrVoid(),
			start: this.decodePositionValue(),
			end: this.decodePositionValue(),
		};
	}

	private decodeArrayBufferView(): RSERArrayBufferView {
		this.readOffset++;

		const code = validateArrayBufferViewCode(this.readInt(1));
		const length = this.decodeNumber();
		const offset = this.decodeNumber();
		const view = this.decodeArrayBuffer();
		return arrayBufferViewCodeToInstance(code, view, offset, length);
	}

	private decodeArrayBuffer(): ArrayBuffer {
		this.readOffset++;
		const offset = this.readOffset;
		const length = this.decodeNumber();
		const buffer: ArrayBuffer = this.bytes.subarray(offset, offset + length);
		this.readOffset += length;
		return buffer;
	}

	private decodeSymbol(): symbol {
		this.readOffset++;
		const key = this.readString();
		return Symbol.for(key);
	}

	private decodeTrue(): true {
		this.readOffset++;
		return true;
	}

	private decodeFalse(): false {
		this.readOffset++;
		return false;
	}

	private decodeNull(): null {
		this.readOffset++;
		return null;
	}

	private decodeNaN(): number {
		this.readOffset++;
		return NaN;
	}

	private decodePositiveInfinity(): number {
		this.readOffset++;
		return Number.POSITIVE_INFINITY;
	}

	private decodeNegativeInfinity(): number {
		this.readOffset++;
		return Number.NEGATIVE_INFINITY;
	}

	private decodeUndefined(): undefined {
		this.readOffset++;
		return undefined;
	}

	private decodeFloat(): number {
		this.readOffset++;
		this.assertReadableSize(8);
		const num = this.view.getFloat64(this.readOffset);
		this.readOffset += 8;
		return num;
	}

	private decodeNegativeZero(): number {
		this.readOffset++;
		return -0;
	}

	private decodeNegativeOne(): number {
		this.readOffset++;
		return -1;
	}

	private decodePositiveZero(): number {
		this.readOffset++;
		return 0;
	}

	private decodePositiveOne(): number {
		this.readOffset++;
		return 1;
	}

	private decodeRegExp(): RegExp {
		this.expectCode(VALUE_CODES.REGEXP);
		const pattern = this.readString();
		const flags = this.readString();
		return new RegExp(pattern, flags);
	}

	private decodeArray(): RSERArray {
		const arr = this.decodeArrayHead();
		return this.decodeArrayElements(arr);
	}

	private decodeArrayHead(): RSERArray {
		this.expectCode(VALUE_CODES.ARRAY);
		const length = this.decodeNumber();
		return new Array(length);
	}

	private decodeArrayElements(arr: RSERArray) {
		for (let i = 0; i < arr.length; ++i) {
			arr[i] = this.decodeValue();
		}
		return arr;
	}

	private decodeObject(): RSERObject {
		this.expectCode(VALUE_CODES.OBJECT);
		return this.decodeObjectValue({});
	}

	private decodeObjectValue(obj: RSERObject): RSERObject {
		const length = this.decodeNumber();
		for (let i = 0; i < length; ++i) {
			const key = this.decodeKey();
			const val = this.decodeValue();
			obj[key] = val;
		}
		return obj;
	}

	private decodeTemplatedObjectArray(): RSERArray {
		// Sometimes we may encode a templated object array to a regular array (like when there's no elements)
		const code = this.peekCode();
		if (code === VALUE_CODES.ARRAY) {
			return this.decodeArray();
		} else {
			const arr = this.decodeTemplatedObjectArrayHead();
			return this.decodeTemplateObjectArrayValues(arr);
		}
	}

	private decodeTemplatedObjectArrayHead(): RSERArray {
		this.expectCode(VALUE_CODES.TEMPLATED_OBJECT_ARRAY);
		const length = this.decodeNumber();
		return new Array(length);
	}

	private decodeTemplateObjectArrayValues(arr: RSERArray) {
		// Decode keys
		const keyCount = this.decodeNumber();
		const keys: string[] = [];
		for (let i = 0; i < keyCount; ++i) {
			keys.push(this.readString());
		}

		// Decode array and objects
		for (let i = 0; i < length; ++i) {
			const obj: RSERObject = {};
			for (let keyidx = 0; keyidx < keys.length; ++keyidx) {
				const val = this.decodeValue();
				const key = keys[keyidx];
				obj[key] = val;
			}
			arr[i] = obj;
		}
		return arr;
	}

	private decodeDate(): Date {
		this.expectCode(VALUE_CODES.DATE);
		const time = this.decodeNumber();
		return new Date(time);
	}

	private decodeFilePathCode(): FILE_CODES {
		return validateFileCode(this.readInt(1));
	}

	private decodeFilePath(): AnyFilePath {
		this.expectCode(VALUE_CODES.FILE_PATH);
		const code = this.decodeFilePathCode();
		const str = this.readString();
		return filePathFromCode(code, str);
	}

	private decodeFilePathMap(): AnyRSERFilePathMap {
		this.expectCode(VALUE_CODES.FILE_PATH_MAP);
		const code = this.decodeFilePathCode();
		const map = filePathMapFromCode(code);
		return this.decodeFilePathMapValue(map);
	}

	private decodeFilePathMapValue(map: AnyRSERFilePathMap): AnyRSERFilePathMap {
		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			const str = this.readString();
			const value = this.decodeValue();
			map.setString(str, value);
		}
		return map;
	}

	private decodeFilePathSet(): AnyFilePathSet {
		this.expectCode(VALUE_CODES.FILE_PATH_SET);

		const code = this.decodeFilePathCode();
		const set = filePathSetFromCode(code);

		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			set.addString(this.readString());
		}
		return set;
	}

	private decodeSet(): RSERSet {
		this.expectCode(VALUE_CODES.SET);
		return this.decodeSetValue(new Set());
	}

	private decodeSetValue(set: RSERSet): RSERSet {
		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			set.add(this.decodeValue());
		}
		return set;
	}

	private decodeMap(): RSERMap {
		this.expectCode(VALUE_CODES.MAP);
		return this.decodeMapValue(new Map());
	}

	private decodeMapValue(map: RSERMap): RSERMap {
		const nitems = this.decodeNumber();
		for (let i = 0; i < nitems; ++i) {
			const key = this.decodeValue();
			const value = this.decodeValue();
			map.set(key, value);
		}
		return map;
	}

	private decodeError(): Error {
		this.readOffset++;

		const errorCode = validateErrorCode(this.readInt(1));
		const message = this.readString();
		const stack = this.decodeStringOrVoid();

		const err = errorCodeToInstance(errorCode);
		err.message = message;
		err.stack = stack;

		// @ts-ignore: Validating these is expensive but we can be confident on the validity
		const nodeProps: StructuredNodeSystemErrorProperties = this.decodeObject();
		setNodeErrorProps(err, nodeProps);

		// @ts-ignore: ^^
		const frames: ErrorFrames = this.decodeTemplatedObjectArray();
		setErrorFrames(err, frames);

		return err;
	}

	private decodeStringOrVoid(): string | undefined {
		const code = this.peekCode();
		switch (code) {
			case VALUE_CODES.UNDEFINED:
			case VALUE_CODES.STRING:
			case VALUE_CODES.REFERENCE:
			case VALUE_CODES.DECLARE_REFERENCE: {
				const value = this.decodeValue();

				if (typeof value !== "string" && typeof value !== "undefined") {
					throw this.unexpected(
						`Expected string or undefined but got a type of ${typeof value}`,
					);
				}

				return value;
			}

			default:
				throw this.unexpected(
					`Expected string or undefined but got ${formatCode(code)}`,
				);
		}
	}

	private decodeKey(): string {
		const size = this.decodeNumber();
		if (sharedCachedKeyDecoder.canBeCached(size)) {
			const str = sharedCachedKeyDecoder.decode(
				this.bytes,
				this.readOffset,
				size,
			);
			this.readOffset += size;
			return str;
		} else {
			return this.readStringSize(size);
		}
	}

	private decodeString(): string {
		this.expectCode(VALUE_CODES.STRING);
		return this.readString();
	}

	public decodeInt(): number | bigint {
		this.assertReadableSize(1);
		const size = this.getDecodeNumberSize();
		this.readOffset += 1;
		return this.readInt(size);
	}

	private decodeNumber(code: VALUE_CODES = this.peekCode()): number {
		switch (code) {
			case VALUE_CODES.INT8:
			case VALUE_CODES.INT16:
			case VALUE_CODES.INT32: {
				const num = this.decodeInt();
				if (typeof num === "bigint") {
					throw this.unexpected("Did not expect a bigint");
				} else {
					return num;
				}
			}

			case VALUE_CODES.FLOAT:
				return this.decodeFloat();

			case VALUE_CODES.NEGATIVE_ONE:
				return this.decodeNegativeOne();

			case VALUE_CODES.POSITIVE_ZERO:
				return this.decodePositiveZero();

			case VALUE_CODES.POSITIVE_ONE:
				return this.decodePositiveOne();

			case VALUE_CODES.POSITIVE_INFINITY:
				return this.decodePositiveInfinity();

			case VALUE_CODES.NEGATIVE_INFINITY:
				return this.decodeNegativeInfinity();

			case VALUE_CODES.NEGATIVE_ZERO:
				return this.decodeNegativeZero();

			case VALUE_CODES.INT64:
				throw this.unexpected(
					"Unexpected bigint, only regular numbers accepted",
				);

			default:
				throw this.unexpected(`${formatCode(code)} is not a valid number code`);
		}
	}

	private getDecodeNumberSize(): IntSize {
		const code = this.peekInt(1);
		switch (code) {
			case VALUE_CODES.NEGATIVE_INFINITY:
			case VALUE_CODES.POSITIVE_INFINITY:
			case VALUE_CODES.NEGATIVE_ZERO:
			case VALUE_CODES.POSITIVE_ZERO:
			case VALUE_CODES.NEGATIVE_ONE:
			case VALUE_CODES.POSITIVE_ONE:
				return 0;

			case VALUE_CODES.INT8:
				return 1;

			case VALUE_CODES.INT16:
				return 2;

			case VALUE_CODES.INT32:
				return 4;

			case VALUE_CODES.INT64:
				return 8;

			default:
				throw this.unexpected(`No int encoding for ${formatCode(code)}`);
		}
	}
}
