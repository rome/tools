/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticAdviceItem, DiagnosticLocation} from './types';
import DiagnosticsProcessor from './DiagnosticsProcessor';
import {SourceMapConsumerCollection} from '@romejs/codec-source-map';
import {MarkupFormatOptions, normalizeMarkup} from '@romejs/string-markup';
import {createBlessedDiagnosticMessage} from './descriptions';

export default class DiagnosticsNormalizer {
  constructor(processor: DiagnosticsProcessor) {
    this.sourceMaps = processor.sourceMaps;
    this.markupOptions = processor.options.markupOptions;
  }

  sourceMaps: SourceMapConsumerCollection;
  markupOptions: undefined | MarkupFormatOptions;

  normalizeFilename(filename: undefined | string): undefined | string {
    const {markupOptions} = this;
    if (markupOptions === undefined || filename === undefined) {
      return filename;
    }
    const {normalizeFilename} = markupOptions;
    if (normalizeFilename === undefined) {
      return filename;
    }

    return normalizeFilename(filename);
  }

  normalizeLocation(location: DiagnosticLocation): DiagnosticLocation {
    const {sourceMaps} = this;

    let {marker, filename, start, end} = location;

    if (filename !== undefined) {
      if (start !== undefined) {
        const resolved = sourceMaps.approxOriginalPositionFor(
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
        const resolved = sourceMaps.approxOriginalPositionFor(
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
    }

    return {
      ...location,
      filename: this.normalizeFilename(filename),
      marker: this.maybeNormalizeMarkup(marker),
      start,
      end,
    };
  }

  normalizeMarkup(markup: string): string {
    return normalizeMarkup(markup, this.markupOptions);
  }

  maybeNormalizeMarkup(markup: undefined | string): undefined | string {
    return markup === undefined ? undefined : this.normalizeMarkup(markup);
  }

  normalizeDiagnosticAdviceItem(
    item: DiagnosticAdviceItem,
  ): DiagnosticAdviceItem {
    const {sourceMaps} = this;

    switch (item.type) {
      case 'frame':
        return {
          ...item,
          location: this.normalizeLocation(item.location),
        };

      case 'list':
        return {
          ...item,
          list: item.list.map((markup) => this.normalizeMarkup(markup)),
        };

      case 'log':
        return {
          ...item,
          text: this.normalizeMarkup(item.text),
        };

      case 'stacktrace':
        return {
          ...item,
          frames: item.frames.map((frame) => {
            const {filename, line, column} = frame;
            if (
              filename === undefined ||
              line === undefined ||
              column === undefined ||
              !sourceMaps.has(filename)
            ) {
              return {
                ...frame,
                filename: this.normalizeFilename(frame.filename),
              };
            }

            const resolved = sourceMaps.approxOriginalPositionFor(
              filename,
              line,
              column,
            );
            if (resolved !== undefined) {
              return {
                ...frame,
                filename: this.normalizeFilename(resolved.source),
                line: resolved.line,
                column: resolved.column,
              };
            }

            return frame;
          }),
        };
    }

    return item;
  }

  normalizeDiagnostic(diag: Diagnostic): Diagnostic {
    const {markupOptions, sourceMaps} = this;

    // Fast path for a common case
    if (markupOptions === undefined && !sourceMaps.hasAny()) {
      return diag;
    }

    const {description} = diag;

    let {advice} = description;
    if (advice !== undefined) {
      advice = advice.map((item) => {
        return this.normalizeDiagnosticAdviceItem(item);
      });
    }

    diag = {
      ...diag,
      label: this.maybeNormalizeMarkup(diag.label),
      location: this.normalizeLocation(diag.location),
      description: {
        ...description,
        message: createBlessedDiagnosticMessage(
          this.normalizeMarkup(description.message.value),
        ),
        advice,
      },
    };

    return diag;
  }
}
