/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@internal/parser-core";
import {Number0, Number1} from "@internal/ob1";
import {JSONPropertyValue} from "@internal/codec-config";
import inspector = require("inspector");

import {InterfaceToObject} from "@internal/typescript-helpers";
import {AnyPath} from "@internal/path";

export type CPUProfile = InterfaceToObject<inspector.Profiler.Profile>;

export type MemorySamples = [number, number][];

export type TraceEvent = {
	ts: number;
	pid: number;
	tid: number;
	name: string;
	dur?: number;
	s?: string;
	id?: number;
	ph?: string;
	cat?: string;
	args?: JSONPropertyValue;
};

export type Profile = {
	pid: number;
	tid: number;
	cpuProfile: CPUProfile;
	memorySamples: MemorySamples;
};

export type CoverageRangeWithMetadata = inspector.Profiler.CoverageRange & {
	kind: LocationRangeKind;
};

export type LocationRangeKind = "branch" | "function" | "expression";

export type CoverageLocationRange = {
	path: AnyPath;
	kind: LocationRangeKind;
	count: number;
	start: Position;
	end: Position;
};

export type CoverageFileStats = {
	covered: number;
	uncovered: number;
	total: number;
	percent: number;
};

export type CoverageFile = {
	path: AnyPath;
	lines: CoverageFileStats;
	branches: CoverageFileStats;
	functions: CoverageFileStats;
};

export type ErrorFrame = {
	typeName: undefined | string;
	functionName: undefined | string;
	methodName: undefined | string;
	path: undefined | AnyPath;
	lineNumber: undefined | Number1;
	columnNumber: undefined | Number0;
	isTopLevel: boolean;
	isAsync: boolean;
	isEval: boolean;
	isNative: boolean;
	isConstructor: boolean;
	resolvedLocation: boolean;
};

export type ErrorFrames = ErrorFrame[];
