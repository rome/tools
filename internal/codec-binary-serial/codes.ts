import {Class} from "@internal/typescript-helpers";
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
	createAbsoluteFilePath,
	createRelativeFilePath,
	createURLFilePath,
	createUnknownFilePath,
} from "@internal/path";
import {AnyRSERFilePathMap} from "@internal/codec-binary-serial/types";

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
			return code;

		default:
			throw new Error(`Invalid value code ${code}`);
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

export function errorCodeToConstructor(code: ERROR_CODES): Class<Error> {
	switch (code) {
		case ERROR_CODES.EVAL:
			return EvalError;

		case ERROR_CODES.RANGE:
			return RangeError;

		case ERROR_CODES.REFERENCE:
			return ReferenceError;

		case ERROR_CODES.SYNTAX:
			return SyntaxError;

		case ERROR_CODES.TYPE:
			return TypeError;

		case ERROR_CODES.URI:
			return URIError;

		case ERROR_CODES.REGULAR:
			return Error;

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
	} else if (map instanceof UnknownFilePathMap) {
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
	} else if (set instanceof UnknownFilePathSet) {
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
	} else if (path instanceof UnknownFilePath) {
		return FILE_CODES.UNKNOWN;
	} else if (path instanceof URLFilePath) {
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
			return createURLFilePath(filename);

		case FILE_CODES.UNKNOWN:
			return createUnknownFilePath(filename);
	}
}

export function filePathMapFromCode(code: FILE_CODES): AnyRSERFilePathMap {
	switch (code) {
		case FILE_CODES.UNKNOWN:
			return new UnknownFilePathMap();

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
			return new UnknownFilePathSet();

		case FILE_CODES.RELATIVE:
			return new RelativeFilePathSet();

		case FILE_CODES.ABSOLUTE:
			return new AbsoluteFilePathSet();

		default:
			throw new Error(`File path code ${code} cannot be a map`);
	}
}
