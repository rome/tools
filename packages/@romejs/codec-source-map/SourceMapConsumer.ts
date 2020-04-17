/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  SourceMap,
  ResolvedLocation,
  ParsedMapping,
  ParsedMappings,
} from './types';
import {decodeVLQ} from './base64';
import {
  number0,
  number1,
  add,
  Number1,
  Number0,
  inc,
  get0,
  dec,
} from '@romejs/ob1';
import {Dict} from '@romejs/typescript-helpers';

export function getParsedMappingKey(line: Number1, column: Number0): string {
  return `${String(line)}:${String(column)}`;
}

type GetMappings = () => ParsedMappings;

export default class SourceMapConsumer {
  constructor(file: string, getMappings: GetMappings) {
    this.file = file;
    this._getMappings = getMappings;
    this.mappings = undefined;
  }

  file: string;
  _getMappings: GetMappings;
  mappings: undefined | ParsedMappings;

  static charIsMappingSeparator(str: string, index: number): boolean {
    const c = str.charAt(index);
    return c === ';' || c === ',';
  }

  static fromJSON(sourceMap: SourceMap): SourceMapConsumer {
    return new SourceMapConsumer(
      sourceMap.file,
      () => SourceMapConsumer.parseMappings(sourceMap),
    );
  }

  static parseMappings(sourceMap: SourceMap): ParsedMappings {
    const rawStr: string = sourceMap.mappings;
    const map: ParsedMappings = new Map();

    let generatedLine = number1;
    let previousGeneratedColumn = number0;
    let previousOriginalLine = number1;
    let previousOriginalColumn = number0;
    let previousSource = 0;
    let previousName = 0;
    let length = rawStr.length;
    let index: number = 0;
    let cachedSegments: Dict<Array<number>> = {};
    let value;

    while (index < length) {
      const char = rawStr[index];
      if (char === ';') {
        generatedLine = inc(generatedLine);
        index++;
        previousGeneratedColumn = number0;
      } else if (char === ',') {
        index++;
      } else {
        const mapping: ParsedMapping = {
          generated: {
            line: generatedLine,
            column: number0,
          },
          original: {
            line: number1,
            column: number0,
          },
          source: undefined,
          name: undefined,
        };

        // Because each offset is encoded relative to the previous one,
        // many segments often have the same encoding. We can exploit this
        // fact by caching the parsed variable length fields of each segment,
        // allowing us to avoid a second parse if we encounter the same
        // segment again.
        let end = index;
        for (; end < length; end++) {
          if (SourceMapConsumer.charIsMappingSeparator(rawStr, end)) {
            break;
          }
        }
        const str = rawStr.slice(index, end);

        let segment = cachedSegments[str];
        if (segment) {
          index += str.length;
        } else {
          segment = [];
          while (index < end) {
            [value, index] = decodeVLQ(rawStr, index);
            segment.push(value);
          }

          if (segment.length === 2) {
            throw new Error('Found a source, but no line and column');
          }

          if (segment.length === 3) {
            throw new Error('Found a source and line, but no column');
          }

          cachedSegments[str] = segment;
        }

        // Generated column
        mapping.generated.column = add(previousGeneratedColumn, segment[0]);
        previousGeneratedColumn = mapping.generated.column;

        if (segment.length > 1) {
          // Original source
          mapping.source = sourceMap.sources[previousSource + segment[1]];
          previousSource += segment[1];

          // Original line
          const newOriginalLine = add(previousOriginalLine, segment[2]);
          previousOriginalLine = newOriginalLine;

          // Lines are stored 0-based
          mapping.original.line = add(newOriginalLine, 1);

          // Original column
          const newOriginalColumn = add(previousOriginalColumn, segment[3]);
          mapping.original.column = newOriginalColumn;
          previousOriginalColumn = newOriginalColumn;

          if (segment.length > 4) {
            // Original name
            mapping.name = sourceMap.names[previousName + segment[4]];
            previousName += segment[4];
          }
        }

        map.set(getParsedMappingKey(
          mapping.generated.line,
          mapping.generated.column,
        ), mapping);
      }
    }

    return map;
  }

  getMappings(): ParsedMappings {
    if (this.mappings === undefined) {
      const mappings = this._getMappings();
      this.mappings = mappings;
      return mappings;
    } else {
      return this.mappings;
    }
  }

  approxOriginalPositionFor(
    line: Number1,
    column: Number0,
  ): undefined | ResolvedLocation {
    while (get0(column) >= 0) {
      const mapping = this.exactOriginalPositionFor(line, column);
      if (mapping === undefined) {
        column = dec(column);
        continue;
      } else {
        return mapping;
      }
    }

    return undefined;
  }

  exactOriginalPositionFor(
    line: Number1,
    column: Number0,
  ): undefined | ResolvedLocation {
    const key = getParsedMappingKey(line, column);
    const mapping = this.getMappings().get(key);
    if (mapping === undefined) {
      return undefined;
    }

    const source = mapping.source === undefined ? this.file : mapping.source;
    if (source === undefined) {
      throw new Error('Mapping provided unknown source');
    }

    return {
      source,
      line: mapping.original.line,
      column: mapping.original.column,
      name: mapping.name,
    };
  }
}
