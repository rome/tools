/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile, TraceEvent} from "./types";
import {urlToFilename} from "./utils";
import {createPath} from "@internal/path";
import {ZeroIndexed} from "@internal/numbers";
import { errorSourceMaps } from "./error-frames";

export default class Trace {
	constructor() {
		this.eventId = 0;
		this.events = [];
	}

	private events: TraceEvent[];
	private eventId: number;

	private getEventId(): number {
		const id = this.eventId;
		this.eventId++;
		return id;
	}

	private decodeProfileSourceMap(profile: Profile) {
		// This method mutates the profile for performance/ergonomics
		// Nothing else should be relying on this so it doesn't really matter
		for (const node of profile.cpuProfile.nodes) {
			const {callFrame} = node;

			// Call frame line numbers are 0-index while Rome is 1-indexed
			const resolved = errorSourceMaps.approxOriginalPositionFor(
				createPath(urlToFilename(callFrame.url)),
				new ZeroIndexed(callFrame.lineNumber).toOneIndexed(),
				new ZeroIndexed(callFrame.columnNumber),
			);
			if (resolved !== undefined) {
				callFrame.url = resolved.source.join();
				callFrame.lineNumber = resolved.line.toZeroIndexed().valueOf();
				callFrame.columnNumber = resolved.column.valueOf();

				if (resolved.name !== undefined) {
					callFrame.functionName = resolved.name;
				}
			}
		}
	}

	public addProfile(name: string, profile: Profile) {
		this.decodeProfileSourceMap(profile);

		const {startTime, endTime} = profile.cpuProfile;

		const common = {
			pid: profile.pid,
			tid: profile.tid,
		};

		this.events.push({
			...common,
			ts: 0,
			ph: "M",
			cat: "__metadata",
			name: "thread_name",
			args: {name},
		});

		this.events.push({
			...common,
			ph: "P",
			name: "CpuProfile",
			id: this.getEventId(),
			cat: "disabled-by-default-v8.cpu_profiler",
			ts: endTime,
			args: {
				data: {
					cpuProfile: profile.cpuProfile,
				},
			},
		});

		this.events.push({
			...common,
			ph: "X",
			name: "EvaluateScript",
			id: this.getEventId(),
			cat: "devtools.timeline",
			ts: startTime,
			dur: endTime - startTime,
			args: {
				data: {
					url: "rome.js",
					lineNumber: 1,
					columnNumber: 1,
					frame: "0xFFF",
				},
			},
		});

		for (const [time, size] of profile.memorySamples) {
			this.events.push({
				...common,
				ts: time,
				ph: "I",
				cat: "disabled-by-default-devtools.timeline",
				name: "UpdateCounters",
				args: {
					data: {
						jsHeapSizeUsed: size,
					},
				},
				s: "t",
			});
		}
	}

	public build() {
		return this.events;
	}
}
