import {
	RSERArray,
	RSERArrayBufferView,
	RSERMap,
	RSERMixedPathMap,
	RSERObject,
	RSERPathMap,
	RSERSet,
	RSERValue,
} from "./types";
import {
	CODES,
	PATH_COLLECTION_CODES,
	PATH_PARSED_CODES,
	VERSION,
	arrayBufferViewCodeToInstance,
	errorCodeToInstance,
	formatCode,
	pathMapFromCode,
	pathSetFromCode,
	validateArrayBufferViewCode,
	validateCode,
	validateErrorCode,
	validatePathCollectionCode,
} from "./codes";
import {
	MixedPathMap,
	MixedPathSet,
	ParsedPath,
	ParsedPathBase,
	ParsedPathDataURI,
	ParsedPathURL,
	Path,
	PathSet,
	createPathFromParsed,
	isPath,
	validateParsedPathWindowsDriveLetter,
} from "@internal/path";
import {
	ErrorFrame,
	NodeSystemErrorProperties,
	setErrorFrames,
	setNodeErrorProps,
} from "@internal/errors";
import {IntSize} from "./utils";
import {utf8Decode} from "@internal/binary";
import {CachedKeyDecoder} from "./CachedKeyDecoder";
import {ExtendedMap} from "@internal/collections";
import RSERParserError from "./RSERParserError";
import {Duration, OneIndexed, ZeroIndexed} from "@internal/numbers";
import {Position, SourceLocation} from "@internal/parser-core";

const sharedCachedKeyDecoder = new CachedKeyDecoder();

export default class RSERBufferParser {
	constructor(view: DataView) {
		this.view = view;
		this.bytes = new Uint8Array(view.buffer, view.byteOffset, view.byteLength);
		this.readOffset = 0;
		this.references = new ExtendedMap("references");

		this.peekedCode = undefined;
		this.peekedCodeOffset = undefined;
	}

	private references: ExtendedMap<number, RSERValue>;
	private view: DataView;
	private bytes: Uint8Array;
	public readOffset: number;

	private peekedCode: undefined | number;
	private peekedCodeOffset: undefined | number;

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

	private peekInt(size: 1, offset?: number): number;
	private peekInt(size: 2, offset?: number): number;
	private peekInt(size: 4, offset?: number): number;
	private peekInt(size: 8, offset?: number): bigint;
	private peekInt(size: IntSize, offset?: number): number | bigint;
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

	private peekCode(): CODES {
		if (
			this.peekedCode !== undefined &&
			this.peekedCodeOffset === this.readOffset
		) {
			return this.peekedCode;
		}

		const code = validateCode(this.peekInt(1));
		this.peekedCode = code;
		this.peekedCodeOffset = this.readOffset;
		return code;
	}

	private readInt(bytes: 1): number;
	private readInt(bytes: 2): number;
	private readInt(bytes: 4): number;
	private readInt(bytes: 8): bigint;
	private readInt(bytes: IntSize): number | bigint;
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
			if (got === CODES.STREAM_HEADER) {
				this.expectCode(CODES.STREAM_HEADER);
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
		if (!this.canRead(1)) {
			return false;
		}

		const messageCode = this.peekCode();
		if (messageCode !== CODES.MESSAGE_HEADER) {
			throw this.unexpected(
				`Unknown message header code ${formatCode(messageCode)}`,
			);
		}
		this.readOffset++;

		const size = this.maybeDecodeNumber();
		if (size === undefined) {
			this.readOffset--;
			return false;
		}

		return size;
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

	private maybeDecodeReference(): undefined | RSERValue {
		const code = this.peekCode();
		if (code === CODES.REFERENCE) {
			return this.decodeReference();
		} else if (code === CODES.DECLARE_REFERENCE) {
			return this.decodeDeclareReference();
		} else {
			return undefined;
		}
	}

	public decodeDeclareReference(): RSERValue {
		this.expectCode(CODES.DECLARE_REFERENCE);
		const id = this.decodeNumber();
		const code = this.peekCode();

		switch (code) {
			case CODES.PATH_MAP: {
				this.readOffset++;
				const code = this.decodePathCollectionCode();
				const map = pathMapFromCode(code);
				this.references.set(id, map);
				return this.decodePathMapValue(map);
			}

			case CODES.MIXED_PATH_MAP: {
				this.readOffset++;
				const map: RSERMixedPathMap = new MixedPathMap();
				this.references.set(id, map);
				return this.decodeMixedPathMapValue(map);
			}

			case CODES.SET: {
				this.readOffset++;
				const set: RSERSet = new Set();
				this.references.set(id, set);
				return this.decodeSetValue(set);
			}

			case CODES.MAP: {
				this.readOffset++;
				const map: RSERMap = new Map();
				this.references.set(id, map);
				return this.decodeMapValue(map);
			}

			case CODES.ARRAY: {
				const arr: RSERArray = this.decodeArrayHead();
				this.references.set(id, arr);
				return this.decodeArrayElements(arr);
			}

			case CODES.TEMPLATED_OBJECT_ARRAY: {
				const arr: RSERArray = this.decodeTemplatedObjectArrayHead();
				this.references.set(id, arr);
				return this.decodeTemplateObjectArrayValues(arr);
			}

			case CODES.OBJECT: {
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
		this.expectCode(CODES.REFERENCE);
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
	private decodeReferentialValue(code: CODES): undefined | RSERValue {
		switch (code) {
			case CODES.PATH_MAP:
				return this.decodePathMap();

			case CODES.SET:
				return this.decodeSet();

			case CODES.MAP:
				return this.decodeMap();

			case CODES.ARRAY:
				return this.decodeArray();

			case CODES.OBJECT:
				return this.decodeObject();

			case CODES.TEMPLATED_OBJECT_ARRAY:
				return this.decodeTemplatedObjectArray();

			case CODES.REFERENCE:
				return this.decodeReference();

			case CODES.DECLARE_REFERENCE:
				return this.decodeDeclareReference();

			default:
				return undefined;
		}
	}

	private decodeNonReferentialValue(code: CODES): RSERValue {
		switch (code) {
			case CODES.INT8:
			case CODES.INT16:
			case CODES.INT32:
			case CODES.FLOAT:
			case CODES.NEGATIVE_ONE:
			case CODES.POSITIVE_ZERO:
			case CODES.POSITIVE_ONE:
			case CODES.POSITIVE_INFINITY:
			case CODES.NEGATIVE_INFINITY:
			case CODES.NEGATIVE_ZERO:
				return this.decodeNumber();

			case CODES.ONE_INDEXED_NUMBER:
				return this.decodeOneIndexedNumber();

			case CODES.ZERO_INDEXED_NUMBER:
				return this.decodeZeroIndexedNumber();

			case CODES.DURATION:
				return this.decodeDuration();

			case CODES.INT64:
				return this.decodeInt();

			case CODES.BIGINT:
				return this.decodeBigInt();

			case CODES.SYMBOL:
				return this.decodeSymbol();

			case CODES.TRUE:
			case CODES.FALSE:
				return this.decodeBoolean();

			case CODES.NULL:
				return this.decodeNull();

			case CODES.UNDEFINED:
				return this.decodeUndefined();

			case CODES.NAN:
				return this.decodeNaN();

			case CODES.PATH:
				return this.decodePath();

			case CODES.PATH_SET:
				return this.decodePathSet();

			case CODES.MIXED_PATH_SET:
				return this.decodeMixedPathSet();

			case CODES.MIXED_PATH_MAP:
				return this.decodeMixedPathMap();

			case CODES.ERROR:
				return this.decodeError();

			case CODES.STRING:
				return this.decodeString();

			case CODES.REGEXP:
				return this.decodeRegExp();

			case CODES.DATE:
				return this.decodeDate();

			case CODES.ARRAY_BUFFER_VIEW:
				return this.decodeArrayBufferView();

			case CODES.ARRAY_BUFFER:
				return this.decodeArrayBuffer();

			case CODES.POSITION:
				return this.decodePosition();

			case CODES.SOURCE_LOCATION:
				return this.decodeSourceLocation();

			default:
				throw this.unexpected(`Unhandled ${formatCode(code)} code`);
		}
	}

	private decodePosition(): Position {
		this.expectCode(CODES.POSITION);
		return this.decodePositionValue();
	}

	private decodePositionValue(): Position {
		return {
			line: new OneIndexed(this.decodeNumber()),
			column: new ZeroIndexed(this.decodeNumber()),
		};
	}

	private decodeSourceLocation(): SourceLocation {
		this.expectCode(CODES.SOURCE_LOCATION);
		return {
			path: this.decodePath(),
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
		return this.decodeArrayBufferValue();
	}

	private decodeArrayBufferValue(): ArrayBuffer {
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

	private decodeBoolean(): boolean {
		const code = this.peekCode();
		switch (code) {
			case CODES.TRUE:
				return this.decodeTrue();

			case CODES.FALSE:
				return this.decodeFalse();

			default:
				throw this.unexpected(`${formatCode(code)} is not a valid boolean code`);
		}
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

	private decodeOneIndexedNumber(): OneIndexed {
		this.expectCode(CODES.ONE_INDEXED_NUMBER);
		return new OneIndexed(this.decodeNumber());
	}

	private decodeZeroIndexedNumber(): ZeroIndexed {
		this.expectCode(CODES.ZERO_INDEXED_NUMBER);
		return new ZeroIndexed(this.decodeNumber());
	}

	private decodeDuration(): Duration {
		this.expectCode(CODES.DURATION);
		return Duration.fromNanoseconds(this.readInt(8));
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
		this.expectCode(CODES.REGEXP);
		const pattern = this.readString();
		const flags = this.readString();
		return new RegExp(pattern, flags);
	}

	private decodeArray(): RSERArray {
		const arr = this.decodeArrayHead();
		return this.decodeArrayElements(arr);
	}

	private decodeArrayHead(): RSERArray {
		this.expectCode(CODES.ARRAY);
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
		this.expectCode(CODES.OBJECT);
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
		if (code === CODES.ARRAY) {
			return this.decodeArray();
		} else {
			const arr = this.decodeTemplatedObjectArrayHead();
			return this.decodeTemplateObjectArrayValues(arr);
		}
	}

	private decodeTemplatedObjectArrayHead(): RSERArray {
		this.expectCode(CODES.TEMPLATED_OBJECT_ARRAY);
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
		for (let i = 0; i < arr.length; ++i) {
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
		this.expectCode(CODES.DATE);
		const time = this.decodeNumber();
		return new Date(time);
	}

	private decodePathCollectionCode(): PATH_COLLECTION_CODES {
		return validatePathCollectionCode(this.readInt(1));
	}

	private decodePath(): Path {
		const ref = this.maybeDecodeReference();
		if (ref !== undefined) {
			if (isPath(ref)) {
				return ref;
			} else {
				throw this.unexpected(
					`Expected path for reference but got a type of ${typeof ref}`,
				);
			}
		}

		return this._decodePath();
	}

	private _decodePath(): Path {
		this.expectCode(CODES.PATH);

		const explicitDirectory = this.decodeBoolean();

		const segmentCount = this.decodeNumber();
		const relativeSegments: string[] = new Array(segmentCount);
		for (let i = 0; i < relativeSegments.length; ++i) {
			relativeSegments[i] = this.decodeKey();
		}

		let parsedBase: ParsedPathBase = {
			relativeSegments,
			explicitDirectory,
		};

		let parsed: ParsedPath;

		const code = this.readInt(1);
		switch (code) {
			case PATH_PARSED_CODES.ABSOLUTE_UNIX: {
				parsed = {
					...parsedBase,
					type: "absolute-unix",
				};
				break;
			}

			case PATH_PARSED_CODES.ABSOLUTE_WINDOWS_DRIVE: {
				const letter = validateParsedPathWindowsDriveLetter(
					String.fromCharCode(this.readInt(1)),
				);
				parsed = {
					...parsedBase,
					type: "absolute-windows-drive",
					letter,
				};
				break;
			}

			case PATH_PARSED_CODES.ABSOLUTE_WINDOWS_UNC: {
				const servername = this.decodeKey();
				parsed = {
					...parsedBase,
					type: "absolute-windows-unc",
					servername,
				};
				break;
			}

			case PATH_PARSED_CODES.RELATIVE: {
				const explicitRelative = this.decodeBoolean();
				parsed = {
					...parsedBase,
					explicitRelative,
					type: "relative",
				};
				break;
			}

			case PATH_PARSED_CODES.URL: {
				let protocol = this.decodeKey();
				let username = this.decodeOptionalKey();
				let password = this.decodeOptionalKey();
				let hostname = this.decodeKey();
				let port = this.decodeNumberOrVoid();

				let search: ParsedPathURL["search"] = new Map();
				const searchSize = this.decodeNumber();
				for (let i = 0; i < searchSize; i++) {
					const key = this.readString();

					const valuesSize = this.decodeNumber();
					const values: string[] = new Array(valuesSize);
					for (let i = 0; i < valuesSize; i++) {
						values[i] = this.readString();
					}

					search.set(key, values);
				}

				let hash = this.decodeOptionalKey();

				parsed = {
					...parsedBase,
					type: "url",
					protocol,
					username,
					password,
					hostname,
					port,
					search,
					hash,
				};
				break;
			}

			case PATH_PARSED_CODES.UID: {
				parsed = {
					...parsedBase,
					type: "uid",
				};
				break;
			}

			case PATH_PARSED_CODES.DATA: {
				const mime = this.decodeOptionalKey();

				let data: ParsedPathDataURI["data"];
				if (this.peekCode() === CODES.ARRAY_BUFFER) {
					data = this.decodeArrayBufferValue();
				} else {
					data = this.decodeString();
				}

				parsed = {
					...parsedBase,
					type: "data",
					mime,
					data,
				};
				break;
			}

			default: {
				throw this.unexpected(`${code} is not a valid parsed path code`);
			}
		}

		return createPathFromParsed(parsed);
	}

	private decodePathMap(): RSERPathMap {
		this.expectCode(CODES.PATH_MAP);
		const code = this.decodePathCollectionCode();
		const map = pathMapFromCode(code);
		return this.decodePathMapValue(map);
	}

	private decodePathMapValue(map: RSERPathMap): RSERPathMap {
		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			const path = this.decodePath();
			const value = this.decodeValue();
			map.setValidated(path, value);
		}
		return map;
	}

	private decodeMixedPathSet(): MixedPathSet {
		this.expectCode(CODES.MIXED_PATH_SET);
		const set = new MixedPathSet();

		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			set.add(this.decodePath());
		}

		return set;
	}

	private decodeMixedPathMap(): RSERMixedPathMap {
		this.expectCode(CODES.MIXED_PATH_MAP);
		const map: RSERMixedPathMap = new MixedPathMap();
		return this.decodeMixedPathMapValue(map);
	}

	private decodeMixedPathMapValue(map: RSERMixedPathMap): RSERMixedPathMap {
		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			const path = this.decodePath();
			const value = this.decodeValue();
			map.set(path, value);
		}
		return map;
	}

	private decodePathSet(): PathSet {
		this.expectCode(CODES.PATH_SET);

		const code = this.decodePathCollectionCode();
		const set = pathSetFromCode(code);

		const size = this.decodeNumber();
		for (let i = 0; i < size; ++i) {
			set.addValidated(this.decodePath());
		}
		return set;
	}

	private decodeSet(): RSERSet {
		this.expectCode(CODES.SET);
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
		this.expectCode(CODES.MAP);
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

		// @ts-expect-error: Validating these is expensive but we can be confident on the validity
		const nodeProps: NodeSystemErrorProperties = this.decodeObject();
		setNodeErrorProps(err, nodeProps);

		// @ts-expect-error: ^^
		const frames: ErrorFrame[] = this.decodeTemplatedObjectArray();
		setErrorFrames(err, frames);

		return err;
	}

	private decodeStringOrVoid(): string | undefined {
		const ref = this.maybeDecodeReference();
		if (ref !== undefined) {
			if (typeof ref === "string") {
				return ref;
			} else {
				throw this.unexpected(
					`Expected string for reference but got a type of ${typeof ref}`,
				);
			}
		}

		const code = this.peekCode();
		switch (code) {
			case CODES.UNDEFINED:
				return this.decodeUndefined();

			case CODES.STRING:
				return this.decodeString();

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

	private decodeOptionalKey(): undefined | string {
		const key = this.decodeKey();
		if (key.length === 0) {
			return undefined;
		} else {
			return key;
		}
	}

	private decodeString(): string {
		this.expectCode(CODES.STRING);
		return this.readString();
	}

	public decodeInt(): number | bigint {
		this.assertReadableSize(1);
		const size = this.getDecodeNumberSize();
		this.readOffset += 1;
		return this.readInt(size);
	}

	public decodeBigInt(): bigint {
		this.expectCode(CODES.BIGINT);
		return BigInt(this.decodeNumber());
	}

	private decodeNumberOrVoid(): undefined | number {
		const code = this.peekCode();
		switch (code) {
			case CODES.UNDEFINED:
				return undefined;

			default:
				return this.decodeNumber();
		}
	}

	private decodeNumber(): number {
		const code = this.peekCode();
		switch (code) {
			case CODES.INT8:
			case CODES.INT16:
			case CODES.INT32: {
				const num = this.decodeInt();
				if (typeof num === "bigint") {
					throw this.unexpected("Did not expect a bigint");
				} else {
					return num;
				}
			}

			case CODES.FLOAT:
				return this.decodeFloat();

			case CODES.NEGATIVE_ONE:
				return this.decodeNegativeOne();

			case CODES.POSITIVE_ZERO:
				return this.decodePositiveZero();

			case CODES.POSITIVE_ONE:
				return this.decodePositiveOne();

			case CODES.POSITIVE_INFINITY:
				return this.decodePositiveInfinity();

			case CODES.NEGATIVE_INFINITY:
				return this.decodeNegativeInfinity();

			case CODES.NEGATIVE_ZERO:
				return this.decodeNegativeZero();

			case CODES.INT64:
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
			case CODES.NEGATIVE_INFINITY:
			case CODES.POSITIVE_INFINITY:
			case CODES.NEGATIVE_ZERO:
			case CODES.POSITIVE_ZERO:
			case CODES.NEGATIVE_ONE:
			case CODES.POSITIVE_ONE:
				return 0;

			case CODES.INT8:
				return 1;

			case CODES.INT16:
				return 2;

			case CODES.INT32:
				return 4;

			case CODES.INT64:
				return 8;

			default:
				throw this.unexpected(`No int encoding for ${formatCode(code)}`);
		}
	}
}
