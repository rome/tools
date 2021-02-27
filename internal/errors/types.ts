import { OneIndexed, ZeroIndexed } from "@internal/numbers";
import { AnyPath } from "@internal/path";

// Similar to TS NodeJS.ErrnoException but with proper properties
// https://nodejs.org/api/errors.html#errors_class_systemerror
export type NodeSystemError = Error &
	Partial<NodeSystemErrorProperties>;

export type NodeSystemErrorProperties = {
	address: undefined | string;
	code: undefined | string;
	dest: undefined | string;
	errno: undefined | number;
	path: undefined | string;
	port: undefined | string;
	syscall: undefined | string;
};

export type ErrorFrame = {
	typeName: undefined | string;
	functionName: undefined | string;
	methodName: undefined | string;
	path: undefined | AnyPath;
	lineNumber: undefined | OneIndexed;
	columnNumber: undefined | ZeroIndexed;
	isTopLevel: boolean;
	isAsync: boolean;
	isEval: boolean;
	isNative: boolean;
	isConstructor: boolean;
	resolvedLocation: boolean;
};

export type ErrorFrames = ErrorFrame[];