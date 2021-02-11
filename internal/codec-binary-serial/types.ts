import {
	AbsoluteFilePathMap,
	AnyPath,
	MixedPathMap,
	PathSet,
	RelativeFilePathMap,
	UIDPathMap,
	URLPathMap,
} from "@internal/path";

export type IntSize = 0 | 1 | 2 | 4 | 8;

export type EqualShapeObjects<Value> = {[key in keyof Value]: Value[key]};

// Value types

export type RSERValueObjects = Extract<RSERValue, object>;

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
	| ArrayBuffer
	| RSERArrayBufferView
	| AnyPath
	| PathSet
	| AnyRSERPathMap
	| RSERMap
	| RSERSet
	| RSERObject
	| RSERArray;

export type AnyRSERPathMap =
	| RSERMixedPathMap
	| AbsoluteFilePathMap<RSERValue>
	| RelativeFilePathMap<RSERValue>
	| URLPathMap<RSERValue>
	| UIDPathMap<RSERValue>;

export type RSERMixedPathMap = MixedPathMap<RSERValue>;

export type RSERMap = Map<RSERValue, RSERValue>;
export type RSERSet = Set<RSERValue>;

export type RSERObject = {
	[x: string]: RSERValue;
};

export type RSERArray = RSERValue[];

// Boring

export type RSERArrayBufferView =
	| Int8Array
	| Uint8Array
	| Uint8ClampedArray
	| Int16Array
	| Uint16Array
	| Int32Array
	| Uint32Array
	| Float32Array
	| Float64Array
	| BigInt64Array
	| BigUint64Array
	| DataView;
