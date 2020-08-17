import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyFilePath,
	AnyFilePathSet,
	RelativeFilePath,
	RelativeFilePathMap,
	RelativeFilePathSet,
	URLPath,
	UnknownPath,
	UnknownPathMap,
	UnknownPathSet,
	createAbsoluteFilePath,
	createRelativeFilePath,
	createURLPath,
	createUnknownPath,
} from "@internal/path";
import {
	AnyRSERFilePathMap,
	RSERArrayBufferView,
} from "@internal/codec-binary-serial/types";

export function formatCode(code: number): string {
	if (VALUE_CODES[code] === undefined) {
		return `?(${code})`;
	} else {
		return `${VALUE_CODES[code]}(${code})`;
	}
}

export enum VALUE_CODES {
	STRING,
	ARRAY,
	SET,
	MAP,
	OBJECT,
	SYMBOL,
	DATE,
	TRUE,
	FALSE,
	NULL,
	UNDEFINED,
	INT8,
	INT16,
	INT32,
	INT64,
	FLOAT,
	NAN,
	POSITIVE_INFINITY,
	NEGATIVE_INFINITY,
	NEGATIVE_ZERO,
	FILE_PATH,
	FILE_PATH_SET,
	FILE_PATH_MAP,
	ERROR,
	REGEXP,
	TEMPLATED_OBJECT_ARRAY,
	REFERENCE,
	DECLARE_REFERENCE,
	ARRAY_BUFFER,
	ARRAY_BUFFER_VIEW,
}

export function validateValueCode(code: number): VALUE_CODES {
	switch (code) {
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
		case VALUE_CODES.FILE_PATH:
		case VALUE_CODES.FILE_PATH_SET:
		case VALUE_CODES.FILE_PATH_MAP:
		case VALUE_CODES.ERROR:
		case VALUE_CODES.REGEXP:
		case VALUE_CODES.TEMPLATED_OBJECT_ARRAY:
		case VALUE_CODES.DECLARE_REFERENCE:
		case VALUE_CODES.REFERENCE:
		case VALUE_CODES.ARRAY_BUFFER_VIEW:
		case VALUE_CODES.ARRAY_BUFFER:
			return code;

		default:
			throw new Error(`Invalid value code ${code}`);
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
			throw new Error(`Invalid typed array code ${code}`);
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
		throw new Error("Unknown typed array instance");
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
			throw new Error(`Invalid error code ${code}`);
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
			throw new Error(`Invalid error code ${code}`);
	}
}

export enum FILE_CODES {
	UNKNOWN,
	ABSOLUTE,
	RELATIVE,
	URL,
}

export function validateFileCode(code: number): FILE_CODES {
	switch (code) {
		case FILE_CODES.UNKNOWN:
		case FILE_CODES.ABSOLUTE:
		case FILE_CODES.RELATIVE:
		case FILE_CODES.URL:
			return code;

		default:
			throw new Error(`Unknown file code ${code}`);
	}
}

export function filePathMapToCode(map: AnyRSERFilePathMap): FILE_CODES {
	if (map instanceof RelativeFilePathMap) {
		return FILE_CODES.RELATIVE;
	} else if (map instanceof AbsoluteFilePathMap) {
		return FILE_CODES.ABSOLUTE;
	} else if (map instanceof UnknownPathMap) {
		return FILE_CODES.UNKNOWN;
	} else {
		throw new Error("Unknown FilePath type");
	}
}

export function filePathSetToCode(set: AnyFilePathSet): FILE_CODES {
	if (set instanceof RelativeFilePathSet) {
		return FILE_CODES.RELATIVE;
	} else if (set instanceof AbsoluteFilePathSet) {
		return FILE_CODES.ABSOLUTE;
	} else if (set instanceof UnknownPathSet) {
		return FILE_CODES.UNKNOWN;
	} else {
		throw new Error("Unknown FilePath type");
	}
}

export function filePathToCode(path: AnyFilePath): FILE_CODES {
	if (path instanceof RelativeFilePath) {
		return FILE_CODES.RELATIVE;
	} else if (path instanceof AbsoluteFilePath) {
		return FILE_CODES.ABSOLUTE;
	} else if (path instanceof UnknownPath) {
		return FILE_CODES.UNKNOWN;
	} else if (path instanceof URLPath) {
		return FILE_CODES.URL;
	} else {
		throw new Error("Unknown FilePath type");
	}
}

export function filePathFromCode(
	code: FILE_CODES,
	filename: string,
): AnyFilePath {
	switch (code) {
		case FILE_CODES.RELATIVE:
			return createRelativeFilePath(filename);

		case FILE_CODES.ABSOLUTE:
			return createAbsoluteFilePath(filename);

		case FILE_CODES.URL:
			return createURLPath(filename);

		case FILE_CODES.UNKNOWN:
			return createUnknownPath(filename);
	}
}

export function filePathMapFromCode(code: FILE_CODES): AnyRSERFilePathMap {
	switch (code) {
		case FILE_CODES.UNKNOWN:
			return new UnknownPathMap();

		case FILE_CODES.RELATIVE:
			return new RelativeFilePathMap();

		case FILE_CODES.ABSOLUTE:
			return new AbsoluteFilePathMap();

		default:
			throw new Error(`File path code ${code} cannot be a map`);
	}
}

export function filePathSetFromCode(code: FILE_CODES): AnyFilePathSet {
	switch (code) {
		case FILE_CODES.UNKNOWN:
			return new UnknownPathSet();

		case FILE_CODES.RELATIVE:
			return new RelativeFilePathSet();

		case FILE_CODES.ABSOLUTE:
			return new AbsoluteFilePathSet();

		default:
			throw new Error(`File path code ${code} cannot be a map`);
	}
}
