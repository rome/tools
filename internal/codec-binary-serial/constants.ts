import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyPath,
	AnyPathSet,
	RelativeFilePath,
	RelativeFilePathMap,
	RelativeFilePathSet,
	UIDPath,
	UIDPathMap,
	UIDPathSet,
	URLPath,
	URLPathMap,
	URLPathSet,
	UnknownPath,
	UnknownPathMap,
	UnknownPathSet,
	createAbsoluteFilePath,
	createRelativeFilePath,
	createUIDPath,
	createURLPath,
	createUnknownPath,
} from "@internal/path";
import {
	AnyRSERPathMap,
	RSERArrayBufferView,
} from "@internal/codec-binary-serial/types";
import RSERParserError from "./RSERParserError";

// Bump whenever we make backwards incompatible changes
export const VERSION = 1;

export function formatCode(code: number): string {
	if (VALUE_CODES[code] === undefined) {
		return `?(${code})`;
	} else {
		return `${VALUE_CODES[code]}(${code})`;
	}
}

export enum VALUE_CODES {
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
	NAN,

	POSITIVE_INFINITY,
	NEGATIVE_INFINITY,
	NEGATIVE_ZERO,

	PATH,
	PATH_SET,
	PATH_MAP,

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

export function validateValueCode(code: number): VALUE_CODES {
	switch (code) {
		case VALUE_CODES.STREAM_HEADER:
		case VALUE_CODES.MESSAGE_HEADER:
		case VALUE_CODES.STRING:
		case VALUE_CODES.ARRAY:
		case VALUE_CODES.SET:
		case VALUE_CODES.MAP:
		case VALUE_CODES.OBJECT:
		case VALUE_CODES.SYMBOL:
		case VALUE_CODES.DATE:
		case VALUE_CODES.TRUE:
		case VALUE_CODES.FALSE:
		case VALUE_CODES.NULL:
		case VALUE_CODES.UNDEFINED:
		case VALUE_CODES.INT8:
		case VALUE_CODES.INT16:
		case VALUE_CODES.INT32:
		case VALUE_CODES.INT64:
		case VALUE_CODES.FLOAT:
		case VALUE_CODES.NAN:
		case VALUE_CODES.POSITIVE_INFINITY:
		case VALUE_CODES.NEGATIVE_INFINITY:
		case VALUE_CODES.NEGATIVE_ZERO:
		case VALUE_CODES.PATH:
		case VALUE_CODES.PATH_SET:
		case VALUE_CODES.PATH_MAP:
		case VALUE_CODES.ERROR:
		case VALUE_CODES.REGEXP:
		case VALUE_CODES.TEMPLATED_OBJECT_ARRAY:
		case VALUE_CODES.DECLARE_REFERENCE:
		case VALUE_CODES.REFERENCE:
		case VALUE_CODES.ARRAY_BUFFER_VIEW:
		case VALUE_CODES.ARRAY_BUFFER:
		case VALUE_CODES.STREAM_HEADER:
		case VALUE_CODES.POSITION:
		case VALUE_CODES.SOURCE_LOCATION:
		case VALUE_CODES.NEGATIVE_ONE:
		case VALUE_CODES.POSITIVE_ZERO:
		case VALUE_CODES.POSITIVE_ONE:
			return code;

		default:
			throw new RSERParserError(`Invalid value code ${code}`);
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
	switch (code) {
		case ARRAY_BUFFER_VIEW_CODES.DATA_VIEW:
		case ARRAY_BUFFER_VIEW_CODES.INT_8:
		case ARRAY_BUFFER_VIEW_CODES.UINT_8:
		case ARRAY_BUFFER_VIEW_CODES.UINT_8_CLAMPED:
		case ARRAY_BUFFER_VIEW_CODES.INT_16:
		case ARRAY_BUFFER_VIEW_CODES.UINT_16:
		case ARRAY_BUFFER_VIEW_CODES.INT_32:
		case ARRAY_BUFFER_VIEW_CODES.UINT_32:
		case ARRAY_BUFFER_VIEW_CODES.FLOAT_32:
		case ARRAY_BUFFER_VIEW_CODES.FLOAT_64:
		case ARRAY_BUFFER_VIEW_CODES.BIG_INT_64:
		case ARRAY_BUFFER_VIEW_CODES.BIG_UINT_64:
			return code;

		default:
			throw new RSERParserError(`Invalid typed array code ${code}`);
	}
}

export function instanceToArrayBufferViewCode(
	val: ArrayBufferView,
): ARRAY_BUFFER_VIEW_CODES {
	if (val instanceof Int8Array) {
		return ARRAY_BUFFER_VIEW_CODES.INT_8;
	} else if (val instanceof Uint8Array) {
		return ARRAY_BUFFER_VIEW_CODES.UINT_8;
	} else if (val instanceof Uint8ClampedArray) {
		return ARRAY_BUFFER_VIEW_CODES.UINT_8_CLAMPED;
	} else if (val instanceof Int16Array) {
		return ARRAY_BUFFER_VIEW_CODES.INT_16;
	} else if (val instanceof Uint16Array) {
		return ARRAY_BUFFER_VIEW_CODES.UINT_16;
	} else if (val instanceof Int32Array) {
		return ARRAY_BUFFER_VIEW_CODES.INT_32;
	} else if (val instanceof Uint32Array) {
		return ARRAY_BUFFER_VIEW_CODES.UINT_32;
	} else if (val instanceof Float32Array) {
		return ARRAY_BUFFER_VIEW_CODES.FLOAT_32;
	} else if (val instanceof Float64Array) {
		return ARRAY_BUFFER_VIEW_CODES.FLOAT_64;
	} else if (val instanceof BigInt64Array) {
		return ARRAY_BUFFER_VIEW_CODES.BIG_INT_64;
	} else if (val instanceof BigUint64Array) {
		return ARRAY_BUFFER_VIEW_CODES.BIG_UINT_64;
	} else if (val instanceof DataView) {
		return ARRAY_BUFFER_VIEW_CODES.DATA_VIEW;
	} else {
		throw new RSERParserError("Unknown typed array instance");
	}
}
export function arrayBufferViewCodeToInstance(
	code: ARRAY_BUFFER_VIEW_CODES,
	buffer: ArrayBuffer,
	offset: number,
	length: number,
): RSERArrayBufferView {
	switch (code) {
		case ARRAY_BUFFER_VIEW_CODES.INT_8:
			return new Int8Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.UINT_8:
			return new Uint8Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.UINT_8_CLAMPED:
			return new Uint8ClampedArray(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.INT_16:
			return new Int16Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.UINT_16:
			return new Uint16Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.INT_32:
			return new Int32Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.UINT_32:
			return new Uint32Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.FLOAT_32:
			return new Float32Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.FLOAT_64:
			return new Float64Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.BIG_INT_64:
			return new BigInt64Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.BIG_UINT_64:
			return new BigUint64Array(buffer, offset, length);

		case ARRAY_BUFFER_VIEW_CODES.DATA_VIEW:
			return new DataView(buffer, offset, length);
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

export function validateErrorCode(code: number): ERROR_CODES {
	switch (code) {
		case ERROR_CODES.REGULAR:
		case ERROR_CODES.EVAL:
		case ERROR_CODES.RANGE:
		case ERROR_CODES.REFERENCE:
		case ERROR_CODES.SYNTAX:
		case ERROR_CODES.TYPE:
		case ERROR_CODES.URI:
			return code;

		default:
			throw new RSERParserError(`Invalid error code ${code}`);
	}
}

export function instanceToErrorCode(err: Error): ERROR_CODES {
	if (err instanceof EvalError) {
		return ERROR_CODES.EVAL;
	} else if (err instanceof ReferenceError) {
		return ERROR_CODES.REFERENCE;
	} else if (err instanceof SyntaxError) {
		return ERROR_CODES.SYNTAX;
	} else if (err instanceof TypeError) {
		return ERROR_CODES.TYPE;
	} else if (err instanceof URIError) {
		return ERROR_CODES.URI;
	} else if (err instanceof RangeError) {
		return ERROR_CODES.RANGE;
	} else {
		return ERROR_CODES.REGULAR;
	}
}

export function errorCodeToInstance(code: ERROR_CODES): Error {
	switch (code) {
		case ERROR_CODES.EVAL:
			return new EvalError();

		case ERROR_CODES.RANGE:
			return new RangeError();

		case ERROR_CODES.REFERENCE:
			return new ReferenceError();

		case ERROR_CODES.SYNTAX:
			return new SyntaxError();

		case ERROR_CODES.TYPE:
			return new TypeError();

		case ERROR_CODES.URI:
			return new URIError();

		case ERROR_CODES.REGULAR:
			return new Error();

		default:
			throw new RSERParserError(`Invalid error code ${code}`);
	}
}

export enum PATH_CODES {
	UNKNOWN,
	ABSOLUTE,
	RELATIVE,
	URL,
	UID,
}

export function validateFileCode(code: number): PATH_CODES {
	switch (code) {
		case PATH_CODES.UNKNOWN:
		case PATH_CODES.ABSOLUTE:
		case PATH_CODES.RELATIVE:
		case PATH_CODES.URL:
		case PATH_CODES.UID:
			return code;

		default:
			throw new RSERParserError(`Unknown path code ${code}`);
	}
}

export function pathMapToCode(map: AnyRSERPathMap): PATH_CODES {
	if (map instanceof RelativeFilePathMap) {
		return PATH_CODES.RELATIVE;
	} else if (map instanceof AbsoluteFilePathMap) {
		return PATH_CODES.ABSOLUTE;
	} else if (map instanceof UnknownPathMap) {
		return PATH_CODES.UNKNOWN;
	} else if (map instanceof URLPathMap) {
		return PATH_CODES.URL;
	} else if (map instanceof UIDPathMap) {
		return PATH_CODES.UID;
	} else {
		throw new RSERParserError("Unknown FilePath type");
	}
}

export function pathSetToCode(set: AnyPathSet): PATH_CODES {
	if (set instanceof RelativeFilePathSet) {
		return PATH_CODES.RELATIVE;
	} else if (set instanceof AbsoluteFilePathSet) {
		return PATH_CODES.ABSOLUTE;
	} else if (set instanceof UnknownPathSet) {
		return PATH_CODES.UNKNOWN;
	} else if (set instanceof URLPathSet) {
		return PATH_CODES.URL;
	} else if (set instanceof UIDPathSet) {
		return PATH_CODES.UID;
	} else {
		throw new RSERParserError("Unknown FilePath type");
	}
}

export function pathToCode(path: AnyPath): PATH_CODES {
	if (path instanceof RelativeFilePath) {
		return PATH_CODES.RELATIVE;
	} else if (path instanceof AbsoluteFilePath) {
		return PATH_CODES.ABSOLUTE;
	} else if (path instanceof UnknownPath) {
		return PATH_CODES.UNKNOWN;
	} else if (path instanceof URLPath) {
		return PATH_CODES.URL;
	} else if (path instanceof UIDPath) {
		return PATH_CODES.UID;
	} else {
		throw new RSERParserError("Unknown FilePath type");
	}
}

export function pathFromCode(code: PATH_CODES, filename: string): AnyPath {
	switch (code) {
		case PATH_CODES.RELATIVE:
			return createRelativeFilePath(filename);

		case PATH_CODES.ABSOLUTE:
			return createAbsoluteFilePath(filename);

		case PATH_CODES.URL:
			return createURLPath(filename);

		case PATH_CODES.UNKNOWN:
			return createUnknownPath(filename);

		case PATH_CODES.UID:
			return createUIDPath(filename);
	}
}

export function pathMapFromCode(code: PATH_CODES): AnyRSERPathMap {
	switch (code) {
		case PATH_CODES.UNKNOWN:
			return new UnknownPathMap();

		case PATH_CODES.RELATIVE:
			return new RelativeFilePathMap();

		case PATH_CODES.ABSOLUTE:
			return new AbsoluteFilePathMap();

		case PATH_CODES.URL:
			return new URLPathMap();

		case PATH_CODES.UID:
			return new UIDPathMap();

		default:
			throw new RSERParserError(`File path code ${code} cannot be a map`);
	}
}

export function pathSetFromCode(code: PATH_CODES): AnyPathSet {
	switch (code) {
		case PATH_CODES.UNKNOWN:
			return new UnknownPathSet();

		case PATH_CODES.RELATIVE:
			return new RelativeFilePathSet();

		case PATH_CODES.ABSOLUTE:
			return new AbsoluteFilePathSet();

		case PATH_CODES.UID:
			return new UIDPathSet();

		default:
			throw new RSERParserError(`File path code ${code} cannot be a map`);
	}
}
