/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile, TraceEvent} from './types';
import * as sourceMapManager from './sourceMapManager';
import {urlToFilename} from './utils';
import {ob1Coerce0, ob1Coerce0To1, ob1Coerce1To0, ob1Get0} from '@romejs/ob1';

export default class Trace {
  constructor() {
    this.tid = 0;
    this.eventId = 0;
    this.events = [];
  }

  events: Array<TraceEvent>;
  tid: number;
  eventId: number;

  getEventId(): number {
    const id = this.eventId;
    this.eventId++;
    return id;
  }

  decodeProfileSourceMap(profile: Profile) {
    // This method mutates the profile for performance/ergonomics
    // Nothing else should be relying on this so it doesn't really matter
    for (const node of profile.cpuProfile.nodes) {
      const {callFrame} = node;
      const filename = urlToFilename(callFrame.url);
      const sourceMap = sourceMapManager.getSourceMap(filename);
      if (sourceMap === undefined) {
        continue;
      }

      // Call frame line numbers are 0-index while Rome is 1-indexed
      const resolved = sourceMap.approxOriginalPositionFor(
        ob1Coerce0To1(ob1Coerce0(callFrame.lineNumber)),
        ob1Coerce0(callFrame.columnNumber),
      );
      if (resolved !== undefined) {
        callFrame.url = resolved.source;
        callFrame.lineNumber = ob1Get0(ob1Coerce1To0(resolved.line));
        callFrame.columnNumber = ob1Get0(resolved.column);

        if (resolved.name !== undefined) {
          callFrame.functionName = resolved.name;
        }
      }
    }
  }

  addProfile(name: string, profile: Profile) {
    this.decodeProfileSourceMap(profile);

    const {startTime, endTime} = profile.cpuProfile;

    const common = {
      pid: 1,
      tid: profile.pid,
    };

    this.events.push({
      ...common,
      ts: 0,
      ph: 'M',
      cat: '__metadata',
      name: 'thread_name',
      args: {name},
    });

    this.events.push({
      ...common,
      ph: 'P',
      name: 'CpuProfile',
      id: this.getEventId(),
      cat: 'disabled-by-default-v8.cpu_profiler',
      ts: endTime,
      args: {
        data: {
          cpuProfile: profile.cpuProfile,
        },
      },
    });

    this.events.push({
      ...common,
      ph: 'X',
      name: 'EvaluateScript',
      id: this.getEventId(),
      cat: 'devtools.timeline',
      ts: startTime,
      dur: endTime - startTime,
      args: {
        data: {
          url: 'rome.js',
          lineNumber: 1,
          columnNumber: 1,
          frame: '0xFFF',
        },
      },
    });

    for (const [time, size] of profile.memorySamples) {
      this.events.push({
        ...common,
        ts: time,
        ph: 'I',
        cat: 'disabled-by-default-devtools.timeline',
        name: 'UpdateCounters',
        args: {
          data: {
            jsHeapSizeUsed: size,
          },
        },
        s: 't',
      });
    }
  }

  build() {
    return this.events;
  }
}
