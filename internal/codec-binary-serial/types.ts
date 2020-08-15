import {
	AbsoluteFilePathMap,
	AnyFilePath,
	AnyFilePathSet,
	RelativeFilePathMap,
	UnknownPathMap,
} from "@internal/path";

export type IntSize = 1 | 2 | 4 | 8;

export type EqualShapeObjects<Value> = {[key in keyof Value]: Value[key]};

export type RSERValue =
	| undefined
	| void
	| null
	| string
	| number
	| bigint
	| boolean
	| symbol
	| Date
	| RegExp
	| Error
	| AnyFilePath
	| AnyFilePathSet
	| AnyRSERFilePathMap
	| RSERMap
	| RSERSet
	| RSERObject
	| RSERArray;

export type AnyRSERFilePathMap =
	| RSERAbsoluteFilePathMap
	| RSERRelativeFilePathMap
	| RSERUnknownPathMap;

export type RSERUnknownPathMap = UnknownPathMap<RSERValue>;
export type RSERAbsoluteFilePathMap = AbsoluteFilePathMap<RSERValue>;
export type RSERRelativeFilePathMap = RelativeFilePathMap<RSERValue>;

export type RSERMap = Map<RSERValue, RSERValue>;
export type RSERSet = Set<RSERValue>;

export type RSERObject = {
	[x: string]: RSERValue;
};
export type RSERArray = Array<RSERValue>;

export type RSERValueObject = Extract<RSERValue, object>;
