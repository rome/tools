/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/*
 * Copyright 2011 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 */

import {Mapping, SourceMap} from './types';
import * as base64 from './base64';
import {compareByGeneratedPositionsInflated, toRelativeUrl} from './util';
import ArraySet from './ArraySet';
import MappingList from './MappingList';
import {Number1, Number0, get1, get0, number0, number1, inc} from '@romejs/ob1';

export default class SourceMapGenerator {
  constructor(
    args: {
      file?: string;
      sourceRoot?: string;
    },
  ) {
    this.file = args.file;
    this.sourceRoot = args.sourceRoot;

    this.sourcesContents = new Map();
    this.map = undefined;
    this.sources = new ArraySet();
    this.names = new ArraySet();
    this.mappings = new MappingList();
  }

  file: undefined | string;
  sourceRoot: undefined | string;
  sources: ArraySet;
  names: ArraySet;
  mappings: MappingList;
  sourcesContents: Map<string, string>;
  map: undefined | SourceMap;

  assertUnlocked() {
    if (this.map !== undefined) {
      throw new Error(
        'Source map has already been materialized, toJSON() should be your final call',
      );
    }
  }

  /**
   * Add a single mapping from 'original source line and column to the generated
   * source's line and column for this source map being created. The mapping
   * object should have the following properties:
   *
   *   - generated: An object with the generated line and column positions.
   *   - original: An object with the original line and column positions.
   *   - source: The original source file (relative to the sourceRoot).
   *   - name: An optional original token name for this mapping.
   */
  addMapping(mapping: Mapping): void {
    this.assertUnlocked();

    const {name, source} = mapping;

    this.validatePosition(
      'generated',
      mapping.generated.line,
      mapping.generated.column,
    );

    if (mapping.original) {
      this.validatePosition(
        'original',
        mapping.original.line,
        mapping.original.column,
      );
    }

    if (source !== undefined) {
      this.sources.add(source);
    }

    if (name !== undefined) {
      this.names.add(name);
    }

    this.mappings.add(mapping);
  }

  /**
   * Set the source content for a source file.
   */
  setSourceContent(source: string, sourceContent: undefined | string): void {
    this.assertUnlocked();

    if (this.sourceRoot !== undefined) {
      source = toRelativeUrl(this.sourceRoot, source);
    }

    if (sourceContent !== undefined) {
      // Add the source content to the _sourcesContents map.
      this.sourcesContents.set(source, sourceContent);
    } else {
      // Remove the source file from the _sourcesContents map.
      this.sourcesContents.delete(source);
    }
  }

  validatePosition(key: string, line: Number1, column: Number0): void {
    if (get1(line) <= 0) {
      throw new Error(`${key} line should be >= 1 but is ${line}`);
    }

    if (get0(column) < 0) {
      throw new Error(`${key} column should be >= 0 but is ${column}`);
    }
  }

  /**
   * Serialize the accumulated mappings in to the stream of base 64 VLQs
   * specified by the source map format.
   */
  serializeMappings(): string {
    let previousGeneratedColumn: Number0 = number0;
    let previousGeneratedLine: Number1 = number1;
    let previousOriginalColumn: Number0 = number0;
    let previousOriginalLine: Number1 = number1;
    let previousName: number = 0;
    let previousSource: number = 0;
    let result: string = '';

    const mappings = this.mappings.toArray();
    for (let i = 0;
    i < mappings.length;
    i++) {
      const mapping = mappings[i];
      let next = '';

      if (mapping.generated.line !== previousGeneratedLine) {
        previousGeneratedColumn = number0;
        while (mapping.generated.line !== previousGeneratedLine) {
          next += ';';
          previousGeneratedLine = inc(previousGeneratedLine);
        }
      } else if (i > 0) {
        if (!compareByGeneratedPositionsInflated(mapping, mappings[i - 1])) {
          continue;
        }
        next += ',';
      }

      next += base64.encodeVLQ(get0(mapping.generated.column) - get0(
        previousGeneratedColumn,
      ));
      previousGeneratedColumn = mapping.generated.column;

      if (mapping.source !== undefined) {
        const sourceIdx = this.sources.indexOf(mapping.source);
        next += base64.encodeVLQ(sourceIdx - previousSource);
        previousSource = sourceIdx;

        if (mapping.original) {
          next += base64.encodeVLQ(get1(mapping.original.line) - get1(
            previousOriginalLine,
          ));
          previousOriginalLine = mapping.original.line;

          next += base64.encodeVLQ(get0(mapping.original.column) - get0(
            previousOriginalColumn,
          ));
          previousOriginalColumn = mapping.original.column;

          if (mapping.name !== undefined) {
            const nameIdx = this.names.indexOf(mapping.name);
            next += base64.encodeVLQ(nameIdx - previousName);
            previousName = nameIdx;
          }
        }

        // TODO: else, assert mapping.name is undefined since it can't be encoded without an original position
      }

      // TODO: else, assert mapping.original is undefined since it can't be encoded without a source
      result += next;
    }

    return result;
  }

  generateSourcesContent(
    sources: Array<string>,
    sourceRoot: undefined | string,
  ): Array<string> {
    return sources.map((source) => {
      if (sourceRoot !== undefined) {
        source = toRelativeUrl(sourceRoot, source);
      }
      const content = this.sourcesContents.get(source);
      if (content === undefined) {
        throw new Error('Expected content');
      }
      return content;
    });
  }

  /**
   * Externalize the source map.
   */
  toJSON(): SourceMap {
    if (this.map !== undefined) {
      return this.map;
    }

    const sources = this.sources.toArray();
    this.map = {
      version: 3,
      file: this.file,
      names: this.names.toArray(),
      mappings: this.serializeMappings(),
      sourceRoot: this.sourceRoot,
      sources,
      sourcesContent: this.generateSourcesContent(sources, this.sourceRoot),
    };
    return this.map;
  }

  toComment(): string {
    const jsonMap = this.toString();
    const base64Map = new Buffer(jsonMap).toString('base64');
    const comment =
      `//# sourceMappingURL=data:application/json;charset=utf-8;base64,${base64Map}`;
    return comment;
  }

  /**
   * Render the source map being generated to a string.
   */
  toString(): string {
    return JSON.stringify(this.toJSON());
  }
}
