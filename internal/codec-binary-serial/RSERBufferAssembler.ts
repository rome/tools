import {
	VALUE_CODES,
	VERSION,
	instanceToArrayBufferViewCode,
	instanceToErrorCode,
	pathMapToCode,
	pathSetToCode,
	pathToCode,
} from "./constants";
import {
	Position,
	SourceLocation,
	isPosition,
	isSourceLocation,
} from "@internal/parser-core";
import {
	AnyRSERPathMap,
	IntSize,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
	RSERValueObject,
} from "./types";
import {UnionToIntersection, isPlainObject} from "@internal/typescript-helpers";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyPath,
	AnyPathSet,
	RelativeFilePath,
	RelativeFilePathMap,
	RelativeFilePathSet,
	URLPath,
	UnknownPath,
	UnknownPathMap,
	UnknownPathSet,
	isPath,
	isPathMap,
	isPathSet,
} from "@internal/path";
import {getErrorStructure} from "@internal/v8";
import {pretty} from "@internal/pretty-format";
import {utf8Count} from "./utf8";
import {ob1Get} from "@internal/ob1";

const MAX_INT8 = 127;
const MAX_INT16 = 32_767;
const MAX_INT32 = 2_147_483_647;

export type RSERBufferAssemblerReferences = Map<RSERValue, number>;

export default class RSERBufferAssembler {
	constructor() {
		this.totalSize = 0;
		this.seenReferenceable = new Set();
		this.references = new Map();
	}

	public totalSize: number;
	public references: RSERBufferAssemblerReferences;
	private seenReferenceable: Set<RSERValue>;

	protected writeCode(code: number) {
		this.totalSize += 1;
		code;
	}

	protected writeByte(value: number) {
		this.totalSize += 1;
		value;
	}

	protected writeInt(value: bigint | number, size: IntSize) {
		this.totalSize += size;
	}

	// When we are writing the buffer, we will insert a header before all values that will be referenced
	// We need to account for that here since we do it after the fact
	protected onReferenceCreate(id: number) {
		this.encodeDeclareReferenceHead(id);
	}

	protected writeFloat(value: number) {
		value;
		this.totalSize += 8;
	}

	protected appendString(buf: string, size: number) {
		this.totalSize += size;
	}

	public appendBytes(buf: Uint8Array) {
		this.totalSize += buf.byteLength;
	}

	public encodeStreamHeader() {
		this.writeByte(VALUE_CODES.STREAM_HEADER);
		this.encodeInt(VERSION);
	}

	public encodeMessageHeader(size: number) {
		this.writeByte(VALUE_CODES.MESSAGE_HEADER);
		this.encodeInt(size);
	}

	private encodeBigInt(val: bigint) {
		this.writeCode(VALUE_CODES.INT64);
		this.writeInt(val, 8);
	}

	private encodeInt(val: bigint | number) {
		if (typeof val === "bigint") {
			return this.encodeBigInt(val);
		}

		if (Object.is(val, -0)) {
			return this.writeCode(VALUE_CODES.NEGATIVE_ZERO);
		}

		if (val === 0) {
			return this.writeCode(VALUE_CODES.POSITIVE_ZERO);
		}

		if (val === 1) {
			return this.writeCode(VALUE_CODES.POSITIVE_ONE);
		}

		if (val === -1) {
			return this.writeCode(VALUE_CODES.NEGATIVE_ONE);
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

	private encodeTemplatedObjectArray<Value extends RSERObject &
		UnionToIntersection<Value>>(arr: Value[]) {
		// More compact form
		if (arr.length === 0) {
			this.writeCode(VALUE_CODES.ARRAY);
			this.encodeInt(0);
			return;
		}

		this.writeCode(VALUE_CODES.TEMPLATED_OBJECT_ARRAY);
		this.encodeInt(arr.length);

		// Encode keys
		const keys: string[] = Object.keys(arr[0]);
		this.encodeInt(keys.length);
		for (const key of keys) {
			this.encodeStringValue(key);
		}

		// Encode entries
		for (const obj of arr) {
			for (const key of keys) {
				const val = obj[key];
				this.encodeValue(val);
			}
		}
	}

	private encodeArray(val: RSERArray) {
		this.writeCode(VALUE_CODES.ARRAY);
		this.encodeInt(val.length);
		for (let i = 0; i < val.length; ++i) {
			this.encodeValue(val[i]);
		}
	}

	private encodeSet(set: RSERSet) {
		this.writeCode(VALUE_CODES.SET);
		this.encodeInt(set.size);
		for (const elem of set) {
			this.encodeValue(elem);
		}
	}

	private encodeMap(map: RSERMap) {
		this.writeCode(VALUE_CODES.MAP);
		this.encodeInt(map.size);
		for (const [key, value] of map) {
			this.encodeValue(key);
			this.encodeValue(value);
		}
	}

	private encodePathMap(map: AnyRSERPathMap) {
		this.writeCode(VALUE_CODES.PATH_MAP);
		this.writeByte(pathMapToCode(map));
		this.encodeInt(map.size);
		for (const [path, value] of map) {
			this.encodeStringValue(path.join());
			this.encodeValue(value);
		}
	}

	private encodePathSet(set: AnyPathSet) {
		this.writeCode(VALUE_CODES.PATH_SET);
		this.writeByte(pathSetToCode(set));
		this.encodeInt(set.size);
		for (const path of set) {
			this.encodeStringValue(path.join());
		}
	}

	private encodePath(path: AnyPath) {
		this.writeCode(VALUE_CODES.PATH);
		this.writeByte(pathToCode(path));
		this.encodeStringValue(path.join());
	}

	private encodeDate(val: Date) {
		this.writeCode(VALUE_CODES.DATE);
		this.encodeInt(val.valueOf());
	}

	private encodeError(val: Error) {
		this.writeCode(VALUE_CODES.ERROR);
		this.writeCode(instanceToErrorCode(val));

		const struct = getErrorStructure(val, 0, false);
		this.encodeStringValue(struct.message ?? "");
		this.encodeValue(struct.stack);
		this.encodePlainObject(struct.node);
		this.encodeTemplatedObjectArray(struct.frames);
	}

	private encodeNull() {
		this.writeCode(VALUE_CODES.NULL);
	}

	private encodeRegExp(regex: RegExp) {
		this.writeCode(VALUE_CODES.REGEXP);
		this.encodeStringValue(regex.source);
		this.encodeStringValue(regex.flags);
	}

	private encodeReference(id: number) {
		this.writeCode(VALUE_CODES.REFERENCE);
		this.encodeInt(id);
	}

	private encodeDeclareReferenceHead(id: number) {
		this.writeCode(VALUE_CODES.DECLARE_REFERENCE);
		this.encodeInt(id);
	}

	private encodeArrayBuffer(val: ArrayBuffer) {
		this.writeCode(VALUE_CODES.ARRAY_BUFFER);
		this.encodeInt(val.byteLength);
		this.appendBytes(new Uint8Array(val));
	}

	private encodeArrayBufferView(val: ArrayBufferView) {
		this.writeCode(VALUE_CODES.ARRAY_BUFFER_VIEW);
		this.writeByte(instanceToArrayBufferViewCode(val));
		this.encodeInt(val.byteLength);
		this.encodeInt(val.byteOffset);
		this.encodeArrayBuffer(val.buffer);
	}

	private encodePossibleReference(val: RSERValue): boolean {
		const refId = this.references.get(val);

		if (refId === undefined) {
			// Is this the second time we've seen this object?
			if (this.seenReferenceable.has(val)) {
				const id = this.references.size;
				this.references.set(val, id);
				this.onReferenceCreate(id);
				this.encodeReference(id);
				return true;
			}
		} else {
			if (this.seenReferenceable.has(val)) {
				// Already a declared reference
				this.encodeReference(refId);
				return true;
			} else {
				// First time we've seen this but we want it to be a reference
				this.encodeDeclareReferenceHead(refId);
			}
		}

		this.seenReferenceable.add(val);
		return false;
	}

	private encodeObject(val: RSERValueObject) {
		const isReference = this.encodePossibleReference(val);
		if (isReference) {
			return;
		}

		if (val instanceof ArrayBuffer) {
			return this.encodeArrayBuffer(val);
		}

		if (ArrayBuffer.isView(val)) {
			return this.encodeArrayBufferView(val);
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

		if (isPath(val)) {
			return this.encodePath(val);
		}

		if (isPathMap(val)) {
			return this.encodePathMap(val);
		}

		if (isPathSet(val)) {
			return this.encodePathSet(val);
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

	private encodePosition(pos: Position) {
		this.writeCode(VALUE_CODES.POSITION);
		this.encodeInt(ob1Get(pos.line));
		this.encodeInt(ob1Get(pos.column));
	}

	private encodeSourceLocation(loc: SourceLocation) {
		this.writeCode(VALUE_CODES.SOURCE_LOCATION);

		if (loc.path === undefined) {
			this.encodeUndefined();
		} else {
			this.encodePath(loc.path);
		}

		// We don't use encodeValue here as we want to allow identifierName to use our reference table
		if (loc.identifierName === undefined) {
			this.encodeUndefined();
		} else {
			this.encodeString(loc.identifierName, true);
		}

		this.encodeInt(ob1Get(loc.start.line));
		this.encodeInt(ob1Get(loc.start.column));
		this.encodeInt(ob1Get(loc.end.line));
		this.encodeInt(ob1Get(loc.end.column));
	}

	private encodePlainObject(val: RSERObject) {
		const keys = Object.keys(val);

		// Dedicated types for common object shapes
		if (keys.length === 2 && isPosition(val)) {
			return this.encodePosition(val);
		}
		if (keys.length <= 4 && isSourceLocation(val)) {
			return this.encodeSourceLocation(val);
		}

		// First pass to compute number of defined keys
		let numKeys = keys.length;
		for (let i = 0; i < keys.length; ++i) {
			const key = keys[i];
			const v = val[key];
			if (typeof v === "undefined") {
				numKeys--;
			}
		}

		this.writeCode(VALUE_CODES.OBJECT);
		this.encodeInt(numKeys);

		for (let i = 0; i < keys.length; ++i) {
			const key = keys[i];
			const v = val[key];
			if (typeof v === "undefined") {
				// Don't include it
				continue;
			}

			this.encodeStringValue(key);
			this.encodeValue(v);
		}
	}

	private encodeUndefined() {
		this.writeCode(VALUE_CODES.UNDEFINED);
	}

	private encodeString(val: string, allowReference?: boolean) {
		if (allowReference && this.encodePossibleReference(val)) {
			return;
		}

		this.writeCode(VALUE_CODES.STRING);
		this.encodeStringValue(val);
	}

	private encodeStringValue(val: string) {
		const byteLength = utf8Count(val);
		this.encodeInt(byteLength);
		this.appendString(val, byteLength);
	}

	private encodeNumber(val: bigint | number) {
		// +Infinity
		if (val === Number.POSITIVE_INFINITY) {
			return this.writeCode(VALUE_CODES.POSITIVE_INFINITY);
		}

		// -Infinity
		if (val === Number.NEGATIVE_INFINITY) {
			return this.writeCode(VALUE_CODES.NEGATIVE_INFINITY);
		}

		if (typeof val === "bigint" || Number.isSafeInteger(val)) {
			this.encodeInt(val);
		} else {
			this.encodeFloat(val);
		}
	}

	private encodeFloat(val: number) {
		this.writeCode(VALUE_CODES.FLOAT);
		this.writeFloat(val);
	}

	public encodeValue(val: RSERValue) {
		switch (typeof val) {
			case "bigint":
			case "number": {
				// NaN
				if (typeof val === "number" && isNaN(val)) {
					return this.writeCode(VALUE_CODES.NAN);
				}

				return this.encodeNumber(val);
			}

			case "undefined":
				return this.encodeUndefined();

			case "string":
				return this.encodeString(val);

			case "boolean":
				return this.writeByte(val ? VALUE_CODES.TRUE : VALUE_CODES.FALSE);

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
