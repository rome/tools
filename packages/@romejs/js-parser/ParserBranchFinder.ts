/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {State} from './tokenizer/state';
import {JSParser, DiagnosticsFatalError} from './parser';

export type ParserBranch<T> = {
  diagnosticsPriority: undefined | number;
  optimal: boolean;
  newDiagnosticCount: number;
  diagnosticCount: number;
  result: T;
  state: State;
};

export type ParserBranchOptions = {
  maxNewDiagnostics?: number;
  diagnosticsPriority?: number;
};

export default class ParserBranchFinder<T> {
  constructor(parser: JSParser) {
    this.parser = parser;
    this.branch = undefined;
    this.picked = false;
  }

  branch: undefined | ParserBranch<T>;
  parser: JSParser;
  picked: boolean;

  hasOptimalBranch(): boolean {
    return this.branch !== undefined && this.branch.optimal;
  }

  hasBranch(): boolean {
    return this.branch !== undefined;
  }

  add(
    callback: (parser: JSParser) => undefined | T,
    opts: ParserBranchOptions = {},
  ): this {
    const topBranch = this.branch;

    // If we already have a branch that produced no errors then no point continuing
    if (topBranch !== undefined && topBranch.optimal) {
      return this;
    }

    const {maxNewDiagnostics, diagnosticsPriority} = opts;
    const {parser} = this;
    const prevState = parser.cloneState();

    parser.pushScope('MAX_NEW_DIAGNOSTICS', maxNewDiagnostics);

    let result;
    try {
      result = callback(parser);
    } catch (err) {
      if (err instanceof DiagnosticsFatalError) {
        parser.setState(prevState);
        return this;
      } else {
        throw err;
      }
    }

    if (result === undefined) {
      parser.setState(prevState);
      return this;
    }

    // We capture the state at this point because it could have been previously changed
    const newState = parser.state;
    parser.popScope('MAX_NEW_DIAGNOSTICS');
    parser.setState(prevState);

    // Verify that we didn't exceed the maxDiagnostics, this should have already been done in Parser#addDiagnostic
    // but do it again as a sanity check. Previously some code caused the state to be manipulated in odd ways
    const newDiagnosticCount = newState.diagnostics.length;
    const prevDiagnosticCount = prevState.diagnostics.length;
    if (
      maxNewDiagnostics !== undefined &&
      newDiagnosticCount - prevDiagnosticCount > maxNewDiagnostics
    ) {
      throw new Error(
        `Max diagnostics unexpectedly exceeded ${maxNewDiagnostics}. Prev: ${prevDiagnosticCount} New: ${newDiagnosticCount}`,
      );
    }

    const branch: ParserBranch<T> = {
      diagnosticsPriority,
      result,
      state: newState,
      newDiagnosticCount: newDiagnosticCount - prevDiagnosticCount,
      diagnosticCount: newDiagnosticCount,
      optimal: newDiagnosticCount === prevDiagnosticCount,
    };

    // Promote this branch to the leader if it's the first, or if it has less diagnostics than the current
    let shouldPromote = false;

    if (topBranch === undefined || branch.optimal) {
      shouldPromote = true;
    } else {
      // Promote if the branch has less diagnostics than the top branch
      if (branch.diagnosticCount < topBranch.diagnosticCount) {
        shouldPromote = true;
      }

      // Promote if we have a priority but the top branch doesn't
      if (
        branch.diagnosticsPriority !== undefined &&
        topBranch.diagnosticsPriority === undefined
      ) {
        shouldPromote = true;
      }

      // Promote if we have a priority, and the top branch does, and we're higher
      if (
        branch.diagnosticsPriority !== undefined &&
        topBranch.diagnosticsPriority !== undefined &&
        branch.diagnosticsPriority > topBranch.diagnosticsPriority
      ) {
        shouldPromote = true;
      }

      // Don't promote if the top branch has a priority but we don't
      if (
        topBranch.diagnosticsPriority !== undefined &&
        branch.diagnosticsPriority === undefined
      ) {
        shouldPromote = false;
      }
    }

    if (shouldPromote) {
      this.branch = branch;
    }

    return this;
  }

  getBranch(): ParserBranch<T> {
    if (this.branch === undefined) {
      throw new Error('No branch');
    } else {
      return this.branch;
    }
  }

  pickOptional(): undefined | T {
    if (this.hasBranch()) {
      return this.pick();
    } else {
      return undefined;
    }
  }

  pick(): T {
    if (this.picked) {
      throw new Error('Already been picked');
    }
    this.picked = true;

    const {parser} = this;
    const branch = this.getBranch();

    const {result, state} = branch;
    parser.setState(state);
    return result;
  }
}
