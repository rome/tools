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
	isPositionish,
	isSourceLocation,
} from "@internal/parser-core";
import {
	AnyRSERPathMap,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
	RSERValueObjects,
} from "./types";
import {IntSize} from "./int";
import {UnionToIntersection, isPlainObject} from "@internal/typescript-helpers";
import {
	AnyPath,
	MixedPathMap,
	MixedPathSet,
	PathSet,
	isPath,
	isPathMap,
	isPathSet,
} from "@internal/path";
import {getErrorStructure} from "@internal/v8";
import {pretty} from "@internal/pretty-format";
import {utf8Count} from "./utf8";
import {
	AnyIndexedNumber,
	OneIndexed,
	ZeroIndexed,
	isIndexedNumberish,
} from "@internal/math";

const MAX_INT8 = 127;
const MAX_INT16 = 32_767;
const MAX_INT32 = 2_147_483_647;

export type RSERWriterBaseReferences = Map<RSERValue, number>;

export default abstract class RSERWriterBase {
	constructor() {
		this.seenReferenceable = new Set();
		this.references = new Map();
	}

	public references: RSERWriterBaseReferences;
	private seenReferenceable: Set<RSERValue>;

	protected abstract writeByte(value: number): void;
	protected abstract writeInt(value: bigint | number, size: IntSize): void;
	protected abstract writeFloat(value: number): void;
	protected abstract writeString(buf: string, size: number): void;
	protected abstract writeBytes(buf: Uint8Array): void;

	// Allow subclasses to opt out of inserting preallocation sizes
	protected encodeSize(value: number): void {
		this.encodeInt(value);
	}

	protected encodeStringValue(val: string) {
		const byteLength = utf8Count(val);
		this.encodeSize(byteLength);
		this.writeString(val, byteLength);
	}

	// When we are writing the buffer, we will insert a header before all values that will be referenced
	// We need to account for that here since we do it after the fact
	protected onReferenceCreate(id: number) {
		this.encodeDeclareReferenceHead(id);
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
		this.writeByte(VALUE_CODES.INT64);
		this.writeInt(val, 8);
	}

	private encodeInt(val: bigint | number) {
		if (typeof val === "bigint") {
			return this.encodeBigInt(val);
		}

		if (Object.is(val, -0)) {
			return this.writeByte(VALUE_CODES.NEGATIVE_ZERO);
		}

		if (val === 0) {
			return this.writeByte(VALUE_CODES.POSITIVE_ZERO);
		}

		if (val === 1) {
			return this.writeByte(VALUE_CODES.POSITIVE_ONE);
		}

		if (val === -1) {
			return this.writeByte(VALUE_CODES.NEGATIVE_ONE);
		}

		const abs = Math.abs(val);
		if (abs <= MAX_INT8) {
			this.writeByte(VALUE_CODES.INT8);
			this.writeInt(val, 1);
		} else if (abs <= MAX_INT16) {
			this.writeByte(VALUE_CODES.INT16);
			this.writeInt(val, 2);
		} else if (abs <= MAX_INT32) {
			this.writeByte(VALUE_CODES.INT32);
			this.writeInt(val, 4);
		} else {
			this.encodeFloat(val);
		}
	}

	private encodeTemplatedObjectArray<Value extends RSERObject &
		UnionToIntersection<Value>>(arr: Value[]) {
		// More compact form
		if (arr.length === 0) {
			this.writeByte(VALUE_CODES.ARRAY);
			this.writeByte(VALUE_CODES.POSITIVE_ZERO);
			return;
		}

		this.writeByte(VALUE_CODES.TEMPLATED_OBJECT_ARRAY);
		this.encodeSize(arr.length);

		// Encode keys
		const keys: string[] = Object.keys(arr[0]);
		this.encodeSize(keys.length);
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
		this.writeByte(VALUE_CODES.ARRAY);
		this.encodeSize(val.length);
		for (let i = 0; i < val.length; ++i) {
			this.encodeValue(val[i]);
		}
	}

	private encodeIndexedNumber(num: AnyIndexedNumber) {
		if (num instanceof OneIndexed) {
			this.writeByte(VALUE_CODES.ONE_INDEXED_NUMBER);
		} else if (num instanceof ZeroIndexed) {
			this.writeByte(VALUE_CODES.ZERO_INDEXED_NUMBER);
		} else {
			throw new Error("Unknown indexed number");
		}
		this.encodeInt(num.valueOf());
	}

	private encodeSet(set: RSERSet) {
		this.writeByte(VALUE_CODES.SET);
		this.encodeSize(set.size);
		for (const elem of set) {
			this.encodeValue(elem);
		}
	}

	private encodeMap(map: RSERMap) {
		this.writeByte(VALUE_CODES.MAP);
		this.encodeSize(map.size);
		for (const [key, value] of map) {
			this.encodeValue(key);
			this.encodeValue(value);
		}
	}

	private encodePathMap(map: AnyRSERPathMap) {
		if (map instanceof MixedPathMap) {
			this.writeByte(VALUE_CODES.MIXED_PATH_MAP);
			this.encodeSize(map.size);
			for (const [path, value] of map) {
				this.encodePath(path);
				this.encodeValue(value);
			}
		} else {
			this.writeByte(VALUE_CODES.PATH_MAP);
			this.writeByte(pathMapToCode(map));
			this.encodeSize(map.size);
			for (const [path, value] of map) {
				this.encodeStringValue(path.join());
				this.encodeValue(value);
			}
		}
	}

	private encodePathSet(set: PathSet) {
		if (set instanceof MixedPathSet) {
			this.writeByte(VALUE_CODES.MIXED_PATH_SET);
			this.encodeSize(set.size);
			for (const path of set) {
				this.encodePath(path);
			}
		} else {
			this.writeByte(VALUE_CODES.PATH_SET);
			this.writeByte(pathSetToCode(set));
			this.encodeSize(set.size);
			for (const path of set) {
				this.encodeStringValue(path.join());
			}
		}
	}

	private encodePath(path: AnyPath) {
		if (this.encodePossibleReference(path)) {
			return;
		}

		this.writeByte(VALUE_CODES.PATH);
		this.writeByte(pathToCode(path));
		this.encodeStringValue(path.join());
	}

	private encodeDate(val: Date) {
		this.writeByte(VALUE_CODES.DATE);
		this.encodeInt(val.valueOf());
	}

	private encodeError(val: Error) {
		this.writeByte(VALUE_CODES.ERROR);
		this.writeByte(instanceToErrorCode(val));

		const struct = getErrorStructure(val, 0, false);
		this.encodeStringValue(struct.message ?? "");
		this.encodeValue(struct.stack);
		this.encodePlainObject(struct.node);
		this.encodeTemplatedObjectArray(struct.frames);
	}

	private encodeNull() {
		this.writeByte(VALUE_CODES.NULL);
	}

	private encodeRegExp(regex: RegExp) {
		this.writeByte(VALUE_CODES.REGEXP);
		this.encodeStringValue(regex.source);
		this.encodeStringValue(regex.flags);
	}

	private encodeReference(id: number) {
		this.writeByte(VALUE_CODES.REFERENCE);
		this.encodeInt(id);
	}

	protected encodeDeclareReferenceHead(id: number) {
		this.writeByte(VALUE_CODES.DECLARE_REFERENCE);
		this.encodeInt(id);
	}

	private encodeArrayBuffer(val: ArrayBuffer) {
		this.writeByte(VALUE_CODES.ARRAY_BUFFER);
		this.encodeSize(val.byteLength);
		this.writeBytes(new Uint8Array(val));
	}

	private encodeArrayBufferView(val: ArrayBufferView) {
		this.writeByte(VALUE_CODES.ARRAY_BUFFER_VIEW);
		this.writeByte(instanceToArrayBufferViewCode(val));
		this.encodeSize(val.byteLength);
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

	private encodeObject(val: RSERValueObjects) {
		if (isPath(val)) {
			return this.encodePath(val);
		}

		const isReference = this.encodePossibleReference(val);
		if (isReference) {
			return;
		}

		if (Array.isArray(val)) {
			return this.encodeArray(val);
		}

		// Weird duck typing for cross-realm objects
		if (
			isPlainObject(val) &&
			val.constructor !== undefined &&
			val.constructor.name === "Object"
		) {
			return this.encodePlainObject(val);
		}

		if (val instanceof Set) {
			return this.encodeSet(val);
		}

		if (val instanceof Map) {
			return this.encodeMap(val);
		}

		if (isPathMap(val)) {
			return this.encodePathMap(val);
		}

		if (isPathSet(val)) {
			return this.encodePathSet(val);
		}

		if (isIndexedNumberish(val)) {
			return this.encodeIndexedNumber(val);
		}

		if (val instanceof Date) {
			return this.encodeDate(val);
		}

		if (val instanceof Error) {
			return this.encodeError(val);
		}

		if (val instanceof RegExp) {
			return this.encodeRegExp(val);
		}

		if (val instanceof ArrayBuffer) {
			return this.encodeArrayBuffer(val);
		}

		if (ArrayBuffer.isView(val)) {
			return this.encodeArrayBufferView(val);
		}

		throw new Error(
			pretty`Don't know how to serialize the object ${val} to RSER`,
		);
	}

	private encodePosition(pos: Position) {
		this.writeByte(VALUE_CODES.POSITION);
		this.encodeInt(pos.line.valueOf());
		this.encodeInt(pos.column.valueOf());
	}

	private encodeSourceLocation(loc: SourceLocation) {
		this.writeByte(VALUE_CODES.SOURCE_LOCATION);
		this.encodePath(loc.path);

		// We don't use encodeValue here as we want to allow identifierName to use our reference table
		if (loc.identifierName === undefined) {
			this.encodeUndefined();
		} else {
			this.encodeString(loc.identifierName, true);
		}

		this.encodeInt(loc.start.line.valueOf());
		this.encodeInt(loc.start.column.valueOf());
		this.encodeInt(loc.end.line.valueOf());
		this.encodeInt(loc.end.column.valueOf());
	}

	private encodePlainObject(val: RSERObject) {
		const keys = Object.keys(val);

		// Dedicated types for common object shapes
		if (keys.length === 2 && isPositionish(val)) {
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

		this.writeByte(VALUE_CODES.OBJECT);
		this.encodeSize(numKeys);

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
		this.writeByte(VALUE_CODES.UNDEFINED);
	}

	private encodeString(val: string, allowReference?: boolean) {
		if (allowReference && this.encodePossibleReference(val)) {
			return;
		}

		this.writeByte(VALUE_CODES.STRING);
		this.encodeStringValue(val);
	}

	private encodeNumber(val: bigint | number) {
		// +Infinity
		if (val === Number.POSITIVE_INFINITY) {
			return this.writeByte(VALUE_CODES.POSITIVE_INFINITY);
		}

		// -Infinity
		if (val === Number.NEGATIVE_INFINITY) {
			return this.writeByte(VALUE_CODES.NEGATIVE_INFINITY);
		}

		if (typeof val === "bigint" || Number.isSafeInteger(val)) {
			this.encodeInt(val);
		} else {
			this.encodeFloat(val);
		}
	}

	private encodeFloat(val: number) {
		this.writeByte(VALUE_CODES.FLOAT);
		this.writeFloat(val);
	}

	public encodeValue(val: RSERValue) {
		switch (typeof val) {
			case "bigint":
			case "number": {
				// NaN
				if (typeof val === "number" && isNaN(val)) {
					return this.writeByte(VALUE_CODES.NAN);
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
				this.writeByte(VALUE_CODES.SYMBOL);
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
