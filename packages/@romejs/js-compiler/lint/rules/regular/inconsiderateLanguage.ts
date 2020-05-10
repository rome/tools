/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {PositionTracker, SourceLocation} from '@romejs/parser-core';
import {ob1Coerce0} from '@romejs/ob1';
import {isIdentifierish} from '@romejs/js-ast-utils';
import {descriptions} from '@romejs/diagnostics';
import inconsiderateLanguage from './inconsiderateLanguage.json';
import {preserveCasing} from '@romejs/string-utils';

// Fast regex for checking if we need to validate a string
const regex = new RegExp(
  inconsiderateLanguage.map((term) => term.word).join('|'),
  'gi',
);

type CheckResult = {
  loc: SourceLocation;
  word: string;
  description: string;
  suggestion: string;
  startIndex: number;
  endIndex: number;
};

function check(
  loc: SourceLocation,
  input: string,
): {
  fixed: string;
  results: Array<CheckResult>;
} {
  let fixed = input;
  if (!regex.test(input)) {
    return {
      fixed,
      results: [],
    };
  }

  const lower = input.toLowerCase();
  const tracker = new PositionTracker(lower, loc.start);
  const results: Array<CheckResult> = [];

  // This is a bit more complicated since we try to do the minimal amount of work
  for (let i = 0; i < lower.length; i++) {
    const char = lower[i];

    for (const {word, description, suggestion} of inconsiderateLanguage) {
      if (char === word[0] && lower.startsWith(word, i)) {
        const wordWithSourceCasing = input.slice(i, i + word.length);

        results.push({
          // We want to preserve the original casing
          word: wordWithSourceCasing,
          description,
          suggestion: preserveCasing(wordWithSourceCasing, suggestion),
          startIndex: i,
          endIndex: i + word.length,
          // Calculate the actual location of this
          loc: {
            ...loc,
            start: tracker.getPositionFromIndex(ob1Coerce0(i)),
            end: tracker.getPositionFromIndex(ob1Coerce0(i + word.length)),
          },
        });

        i += word.length;
        break;
      }
    }
  }

  // Walk backwards through the results, autofixing with the suggestions
  // Walking backwards means we don't need to maintain offsets
  for (let i = results.length - 1; i >= 0; i--) {
    const result = results[i];
    fixed =
      fixed.slice(0, result.startIndex) +
      result.suggestion +
      fixed.slice(result.endIndex);
  }

  return {
    results,
    fixed,
  };
}

export default {
  name: 'inconsiderateLanguage',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    const {loc} = node;
    if (loc !== undefined) {
      // Infer a string to check
      let value: undefined | string;
      if (node.type === 'CommentBlock' || node.type === 'CommentLine') {
        value = node.value;
      }
      if (isIdentifierish(node)) {
        value = node.name;
      }

      if (value !== undefined) {
        // Produce diagnostics
        const {results, fixed} = check(loc, value);
        let suppressed = false;
        for (const {loc, word, description, suggestion} of results) {
          ({suppressed} = context.addLocDiagnostic(
            loc,
            descriptions.LINT.INCONSIDERATE_LANGUAGE(
              description,
              word,
              suggestion,
            ),
            {fixable: true},
          ));

          if (suppressed) {
            break;
          }
        }

        // Autofix if not suppressed
        if (results.length > 0 && !suppressed) {
          if (node.type === 'CommentBlock' || node.type === 'CommentLine') {
            return {
              ...node,
              value: fixed,
            };
          }

          if (isIdentifierish(node)) {
            return {
              ...node,
              name: fixed,
            };
          }
        }
      }
    }

    return node;
  },
};
