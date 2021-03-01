import {AnyIndexedNumber, Duration} from "@internal/numbers";
import {
	AbsoluteFilePathMap,
	DataURIPathMap,
	MixedPathMap,
	Path,
	PathSet,
	RelativePathMap,
	UIDPathMap,
	URLPathMap,
} from "@internal/path";

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
	| Buffer
	| ArrayBuffer
	| AnyIndexedNumber
	| Duration
	| RSERArrayBufferView
	| Path
	| PathSet
	| AnyRSERPathMap
	| RSERMap
	| RSERSet
	| RSERObject
	| RSERArray;

export type AnyRSERPathMap =
	| RSERMixedPathMap
	| AbsoluteFilePathMap<RSERValue>
	| RelativePathMap<RSERValue>
	| URLPathMap<RSERValue>
	| UIDPathMap<RSERValue>
	| DataURIPathMap<RSERValue>;

export type RSERMixedPathMap = MixedPathMap<RSERValue>;

export type RSERMap = Map<RSERValue, RSERValue>;
export type RSERSet = Set<RSERValue>;

export type RSERObject = {
	[x: string]: RSERValue;
};

export type RSERArray = RSERValue[];

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
