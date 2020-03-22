/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Diagnostics,
  PartialDiagnostics,
  PartialDiagnostic,
  DiagnosticFilterWithTest,
  DiagnosticOrigin,
  DiagnosticSuppressions,
  DiagnosticSuppression,
} from './types';
import {MarkupFormatOptions} from '@romejs/string-markup';
import {addOriginsToDiagnostics} from './derive';
import {naturalCompare} from '@romejs/string-utils';
import {DiagnosticsError} from './errors';
import {normalizeDiagnostics} from './normalize';
import {add} from '@romejs/ob1';
import {DiagnosticCategoryPrefix} from './categories';

type UniquePart =
  | 'filename'
  | 'message'
  | 'start.line'
  | 'start.column'
  | 'category';

type UniqueRule = Array<UniquePart>;

type UniqueRules = Array<UniqueRule>;

export type CollectorOptions = {
  filters?: Array<DiagnosticFilterWithTest>;
  unique?: UniqueRules;
  max?: number;
  onDiagnostics?: (diags: PartialDiagnostics) => void;
  origins?: Array<DiagnosticOrigin>;
};

const DEFAULT_UNIQUE: UniqueRules = [
  ['category', 'filename', 'message', 'start.line', 'start.column'],
];

export default class DiagnosticsProcessor {
  constructor(options: CollectorOptions) {
    this.diagnostics = [];
    this.filters = [];
    this.allowedUnusedSuppressionPrefixes = new Set();
    this.usedSuppressions = new Set();
    this.suppressions = new Set();
    this.options = options;
    this.includedKeys = new Set();
    this.unique = options.unique === undefined ? DEFAULT_UNIQUE : options.unique;
    this.throwAfter = undefined;
  }

  static createImmediateThrower(
    origins: Array<DiagnosticOrigin>,
  ): DiagnosticsProcessor {
    const diagnostics = new DiagnosticsProcessor({
      origins,
      onDiagnostics() {
        diagnostics.maybeThrowDiagnosticsError();
      },
    });
    return diagnostics;
  }

  unique: UniqueRules;
  includedKeys: Set<string>;
  diagnostics: PartialDiagnostics;
  filters: Array<DiagnosticFilterWithTest>;
  allowedUnusedSuppressionPrefixes: Set<string>;
  usedSuppressions: Set<DiagnosticSuppression>;
  suppressions: Set<DiagnosticSuppression>;
  options: CollectorOptions;
  throwAfter: undefined | number;

  setThrowAfter(num: undefined | number) {
    this.throwAfter = num;
  }

  maybeThrowDiagnosticsError() {
    if (this.hasDiagnostics()) {
      throw new DiagnosticsError(
        'Thrown by DiagnosticsProcessor',
        this.getPartialDiagnostics(),
      );
    }
  }

  hasDiagnostics(): boolean {
    return this.diagnostics.length > 0;
  }

  addAllowedUnusedSuppressionPrefix(prefix: DiagnosticCategoryPrefix) {
    this.allowedUnusedSuppressionPrefixes.add(prefix);
  }

  addSuppressions(suppressions: DiagnosticSuppressions) {
    for (const suppression of suppressions) {
      this.suppressions.add(suppression);
    }
  }

  addFilters(filters: Array<DiagnosticFilterWithTest>) {
    this.filters = this.filters.concat(filters);
  }

  addFilter(filter: DiagnosticFilterWithTest) {
    this.filters.push(filter);
  }

  doesMatchFilter(diag: PartialDiagnostic): boolean {
    for (const suppression of this.suppressions) {
      const targetLine = add(suppression.loc.end.line, 1);
      if (diag.filename !== undefined && diag.start !== undefined &&
        diag.filename === suppression.loc.filename && diag.start.line ===
      targetLine) {
        this.usedSuppressions.add(suppression);
        return true;
      }
    }

    for (const filter of this.filters) {
      if (filter.message !== undefined && filter.message !== diag.message) {
        continue;
      }

      if (filter.filename !== undefined && filter.filename !== diag.filename) {
        continue;
      }

      if (filter.category !== undefined && filter.category !== diag.category) {
        continue;
      }

      if (filter.start !== undefined && diag.start !== undefined) {
        if (filter.start.line !== diag.start.line || filter.start.column !==
        diag.start.column) {
          continue;
        }
      }

      if (filter.line !== undefined && diag.start !== undefined &&
        diag.start.line !== filter.line) {
        continue;
      }

      if (filter.test !== undefined && !filter.test(diag)) {
        continue;
      }

      return true;
    }

    return false;
  }

  buildDedupeKeys(diag: PartialDiagnostic): Array<string> {
    // We don't do anything with `end` in this method, it's fairly meaningless for deduping errors
    let {start} = diag;

    const keys: Array<string> = [];

    for (const rule of this.unique) {
      const parts = [];

      if (rule.includes('category')) {
        parts.push(`category:${diag.category}`);
      }

      if (rule.includes('filename')) {
        parts.push(`filename:${String(diag.filename)}`);
      }

      if (rule.includes('message')) {
        parts.push(`message:${diag.message}`);
      }

      if (start !== undefined) {
        if (rule.includes('start.line')) {
          parts.push(`start.line:${start.line}`);
        }

        if (rule.includes('start.column')) {
          parts.push(`start.column:${start.column}`);
        }
      }

      const key = parts.join(',');
      keys.push(key);
    }

    return keys;
  }

  addDiagnostic(diag: PartialDiagnostic, origin?: DiagnosticOrigin): boolean {
    return this.addDiagnostics([diag], origin).length > 0;
  }

  addDiagnostics(
    diags: PartialDiagnostics,
    origin?: DiagnosticOrigin,
  ): PartialDiagnostics {
    if (diags.length === 0) {
      return diags;
    }

    const {max} = this.options;
    const added: PartialDiagnostics = [];

    // Add origins to diagnostics
    const origins: Array<DiagnosticOrigin> = this.options.origins === undefined
      ? [] : [...this.options.origins];
    if (origin !== undefined) {
      origins.push(origin);
    }
    diags = addOriginsToDiagnostics(origins, diags);

    // Filter diagnostics
    diagLoop: for (const diag of diags) {
      if (max !== undefined && this.diagnostics.length > max) {
        break;
      }

      if (this.doesMatchFilter(diag)) {
        continue;
      }

      const keys = this.buildDedupeKeys(diag);

      for (const key of keys) {
        if (this.includedKeys.has(key)) {
          continue diagLoop;
        }
      }

      this.diagnostics.push(diag);
      added.push(diag);

      for (const key of keys) {
        this.includedKeys.add(key);
      }
    }

    const {onDiagnostics} = this.options;
    if (onDiagnostics !== undefined && added.length > 0) {
      onDiagnostics(added);
    }

    const {throwAfter} = this;
    if (throwAfter !== undefined && this.diagnostics.length >= throwAfter) {
      this.maybeThrowDiagnosticsError();
    }

    return added;
  }

  getPartialDiagnostics(): PartialDiagnostics {
    const diagnostics: PartialDiagnostics = [...this.diagnostics];

    // Add errors for remaining suppressions
    for (const suppression of this.suppressions) {
      if (this.usedSuppressions.has(suppression)) {
        continue;
      }

      const [categoryPrefix] = suppression.category.split('/');
      if (this.allowedUnusedSuppressionPrefixes.has(categoryPrefix)) {
        continue;
      }

      diagnostics.push({
        ...suppression.loc,
        message: 'Did not hide any error',
        category: 'suppressions/unused',
      });
    }

    return diagnostics;
  }

  getCompleteDiagnostics(markupOptions: MarkupFormatOptions = {}): Diagnostics {
    return normalizeDiagnostics(this.getPartialDiagnostics(), markupOptions);
  }

  getCompleteSortedDiagnostics(
    markupOptions: MarkupFormatOptions = {},
  ): Diagnostics {
    // Sort files by filename to ensure they're always in the same order

    // TODO also sort by line/column
    return this.getCompleteDiagnostics(markupOptions).sort((a, b) => {
      if (a.filename === undefined || b.filename === undefined) {
        return 0;
      } else {
        return naturalCompare(a.filename, b.filename);
      }
    });
  }

  clear() {
    this.includedKeys = new Set();
    this.diagnostics = [];
  }
}
