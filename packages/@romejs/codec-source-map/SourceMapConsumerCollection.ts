/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import SourceMapConsumer from './SourceMapConsumer';
import {Number1, Number0} from '@romejs/ob1';
import {ResolvedLocation} from './types';
import {DiagnosticLocation} from '@romejs/diagnostics';

export default class SourceMapConsumerCollection {
  constructor() {
    this.maps = new Map();
  }

  maps: Map<string, SourceMapConsumer>;

  hasAny(): boolean {
    return this.maps.size > 0;
  }

  has(file: string): boolean {
    return this.maps.has(file);
  }

  add(file: string, map: SourceMapConsumer) {
    this.maps.set(file, map);
  }

  get(file: string): undefined | SourceMapConsumer {
    return this.maps.get(file);
  }

  resolveLocation(location: DiagnosticLocation): DiagnosticLocation {
    let {filename, start, end} = location;
    if (filename === undefined) {
      return location;
    }

    if (start !== undefined) {
      const resolved = this.approxOriginalPositionFor(
        filename,
        start.line,
        start.column,
      );
      if (resolved !== undefined) {
        filename = resolved.source;
        start = {
          ...start,
          line: resolved.line,
          column: resolved.column,
        };
      }
    }

    if (end !== undefined) {
      const resolved = this.approxOriginalPositionFor(
        filename,
        end.line,
        end.column,
      );
      if (resolved !== undefined) {
        // TODO confirm this is the same as `start` if it resolved
        filename = resolved.source;
        end = {
          ...end,
          line: resolved.line,
          column: resolved.column,
        };
      }
    }

    return {...location, filename, start, end};
  }

  approxOriginalPositionFor(
    file: string,
    line: Number1,
    column: Number0,
  ): undefined | ResolvedLocation {
    const map = this.get(file);
    if (map === undefined) {
      return undefined;
    } else {
      return map.approxOriginalPositionFor(line, column);
    }
  }

  exactOriginalPositionFor(
    file: string,
    line: Number1,
    column: Number0,
  ): undefined | ResolvedLocation {
    const map = this.get(file);
    if (map === undefined) {
      return undefined;
    } else {
      return map.exactOriginalPositionFor(line, column);
    }
  }
}
