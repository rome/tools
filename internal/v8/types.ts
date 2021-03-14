/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@internal/parser-core";
import {JSONPropertyValue} from "@internal/codec-config";
import inspector = require("inspector");

import {InterfaceToObject} from "@internal/typescript-helpers";
import {Path} from "@internal/path";

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
	path: Path;
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
	path: Path;
	lines: CoverageFileStats;
	branches: CoverageFileStats;
	functions: CoverageFileStats;
};
