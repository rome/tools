import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	DataURIPath,
	DataURIPathMap,
	DataURIPathSet,
	Path,
	PathSet,
	RelativePath,
	RelativePathMap,
	RelativePathSet,
	UIDPath,
	UIDPathMap,
	UIDPathSet,
	URLPath,
	URLPathMap,
	URLPathSet,
} from "@internal/path";
import {RSERPathMap, RSERArrayBufferView} from "./types";
import RSERParserError from "./RSERParserError";
import {Class, isSafeInstanceof} from "@internal/typescript-helpers";

// Bump whenever we make backwards incompatible changes
export const VERSION = 1;

export function formatCode(code: number): string {
	if (CODES[code] === undefined) {
		return `?(${code})`;
	} else {
		return `${CODES[code]}(${code})`;
	}
}

export enum CODES {
	STREAM_HEADER,
	MESSAGE_HEADER,

	STRING,
	TRUE,
	FALSE,
	NULL,
	UNDEFINED,

	SYMBOL,
	DATE,
	ERROR,
	REGEXP,

	ARRAY,
	SET,
	MAP,
	OBJECT,
	TEMPLATED_OBJECT_ARRAY,

	INT8,
	INT16,
	INT32,
	INT64,
	FLOAT,
	BIGINT,
	NAN,

	ZERO_INDEXED_NUMBER,
	ONE_INDEXED_NUMBER,
	DURATION,

	POSITIVE_INFINITY,
	NEGATIVE_INFINITY,
	NEGATIVE_ZERO,

	PATH,
	PATH_SET,
	PATH_MAP,
	MIXED_PATH_SET,
	MIXED_PATH_MAP,

	REFERENCE,
	DECLARE_REFERENCE,

	ARRAY_BUFFER,
	ARRAY_BUFFER_VIEW,

	POSITION,
	SOURCE_LOCATION,

	// These save a single byte having to specify an Int8...
	NEGATIVE_ONE,
	POSITIVE_ZERO,
	POSITIVE_ONE,
}

export function validateCode(code: number): CODES {
	const id = CODES[code];
	if (id === undefined) {
		throw new RSERParserError(`Invalid code ${code}`);
	} else {
		return code as CODES;
	}
}

export enum ARRAY_BUFFER_VIEW_CODES {
	DATA_VIEW,
	INT_8,
	UINT_8,
	UINT_8_CLAMPED,
	INT_16,
	UINT_16,
	INT_32,
	UINT_32,
	FLOAT_32,
	FLOAT_64,
	BIG_INT_64,
	BIG_UINT_64,
}

export function validateArrayBufferViewCode(
	code: number,
): ARRAY_BUFFER_VIEW_CODES {
	const id = ARRAY_BUFFER_VIEW_CODES[code];
	if (id === undefined) {
		throw new RSERParserError(`Invalid typed array code ${code}`);
	} else {
		return code as ARRAY_BUFFER_VIEW_CODES;
	}
}

const arrayBufferViewTypes: Map<
	ARRAY_BUFFER_VIEW_CODES,
	{
		new (
			buffer: ArrayBufferLike,
			byteOffset?: number,
			length?: number,
		): RSERArrayBufferView;
	}
> = new Map();
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.INT_8, Int8Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.UINT_8, Uint8Array);
arrayBufferViewTypes.set(
	ARRAY_BUFFER_VIEW_CODES.UINT_8_CLAMPED,
	Uint8ClampedArray,
);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.INT_16, Int16Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.UINT_16, Uint16Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.INT_32, Int32Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.UINT_32, Uint32Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.FLOAT_32, Float32Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.FLOAT_64, Float64Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.BIG_INT_64, BigInt64Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.BIG_UINT_64, BigUint64Array);
arrayBufferViewTypes.set(ARRAY_BUFFER_VIEW_CODES.DATA_VIEW, DataView);

export function instanceToArrayBufferViewCode(
	val: ArrayBufferView,
): ARRAY_BUFFER_VIEW_CODES {
	for (const [code, BufferView] of arrayBufferViewTypes) {
		if (isSafeInstanceof(val, BufferView)) {
			return code;
		}
	}
	throw new RSERParserError("Unknown typed array instance");
}

export function arrayBufferViewCodeToInstance(
	code: ARRAY_BUFFER_VIEW_CODES,
	buffer: ArrayBuffer,
	offset: number,
	length: number,
): RSERArrayBufferView {
	const BufferView = arrayBufferViewTypes.get(code);
	if (BufferView === undefined) {
		throw new RSERParserError(`Unknown ArrayBufferView code ${code}`);
	} else {
		return new BufferView(buffer, offset, length);
	}
}

export enum ERROR_CODES {
	REGULAR,
	EVAL,
	RANGE,
	REFERENCE,
	SYNTAX,
	TYPE,
	URI,
}

const errorTypes: Map<ERROR_CODES, Class<Error>> = new Map();
errorTypes.set(ERROR_CODES.REGULAR, Error);
errorTypes.set(ERROR_CODES.EVAL, EvalError);
errorTypes.set(ERROR_CODES.REFERENCE, ReferenceError);
errorTypes.set(ERROR_CODES.SYNTAX, SyntaxError);
errorTypes.set(ERROR_CODES.TYPE, TypeError);
errorTypes.set(ERROR_CODES.URI, URIError);
errorTypes.set(ERROR_CODES.RANGE, RangeError);

export function validateErrorCode(code: number): ERROR_CODES {
	const id = ERROR_CODES[code];
	if (id === undefined) {
		throw new RSERParserError(`Invalid error code ${code}`);
	} else {
		return code as ERROR_CODES;
	}
}

export function instanceToErrorCode(err: Error): ERROR_CODES {
	for (const [code, ErrorClass] of errorTypes) {
		if (err.name === ErrorClass.name || isSafeInstanceof(err, ErrorClass)) {
			return code;
		}
	}
	return ERROR_CODES.REGULAR;
}

export function errorCodeToInstance(code: ERROR_CODES): Error {
	const ErrorClass = errorTypes.get(code);
	if (ErrorClass === undefined) {
		throw new RSERParserError(`Invalid error code ${code}`);
	} else {
		return new ErrorClass();
	}
}

export enum PATH_PARSED_CODES {
	ABSOLUTE_UNIX,
	ABSOLUTE_WINDOWS_DRIVE,
	ABSOLUTE_WINDOWS_UNC,
	RELATIVE,
	URL,
	DATA,
	UID,
}

export enum PATH_COLLECTION_CODES {
	ABSOLUTE,
	RELATIVE,
	URL,
	DATA,
	UID,
}

const pathCollectionTypes: Map<
	PATH_COLLECTION_CODES,
	{
		Path: Class<Path>;
		PathMap: Class<RSERPathMap>;
		PathSet: Class<PathSet>;
	}
> = new Map();

pathCollectionTypes.set(
	PATH_COLLECTION_CODES.ABSOLUTE,
	{
		Path: AbsoluteFilePath,
		PathMap: AbsoluteFilePathMap,
		PathSet: AbsoluteFilePathSet,
	},
);

pathCollectionTypes.set(
	PATH_COLLECTION_CODES.RELATIVE,
	{
		Path: RelativePath,
		PathMap: RelativePathMap,
		PathSet: RelativePathSet,
	},
);

pathCollectionTypes.set(
	PATH_COLLECTION_CODES.URL,
	{
		Path: URLPath,
		PathMap: URLPathMap,
		PathSet: URLPathSet,
	},
);

pathCollectionTypes.set(
	PATH_COLLECTION_CODES.DATA,
	{
		Path: DataURIPath,
		PathMap: DataURIPathMap,
		PathSet: DataURIPathSet,
	},
);

pathCollectionTypes.set(
	PATH_COLLECTION_CODES.UID,
	{
		Path: UIDPath,
		PathMap: UIDPathMap,
		PathSet: UIDPathSet,
	},
);

export function validatePathCollectionCode(code: number): PATH_COLLECTION_CODES {
	if (pathCollectionTypes.has(code)) {
		return code;
	} else {
		throw new RSERParserError(`Unknown path code ${code}`);
	}
}

export function pathMapToCode(map: RSERPathMap): PATH_COLLECTION_CODES {
	for (const [code, {PathMap}] of pathCollectionTypes) {
		if (isSafeInstanceof(map, PathMap)) {
			return code;
		}
	}
	throw new RSERParserError("Unknown Path type");
}

export function pathSetToCode(set: PathSet): PATH_COLLECTION_CODES {
	for (const [code, {PathSet}] of pathCollectionTypes) {
		if (isSafeInstanceof(set, PathSet)) {
			return code;
		}
	}
	throw new RSERParserError("Unknown Path type");
}

export function pathMapFromCode(code: PATH_COLLECTION_CODES): RSERPathMap {
	const types = pathCollectionTypes.get(code);
	if (types === undefined) {
		throw new RSERParserError(`Unknown path code ${code}`);
	} else {
		return new types.PathMap();
	}
}

export function pathSetFromCode(code: PATH_COLLECTION_CODES): PathSet {
	const types = pathCollectionTypes.get(code);
	if (types === undefined) {
		throw new RSERParserError(`Unknown path code ${code}`);
	} else {
		return new types.PathSet();
	}
}
