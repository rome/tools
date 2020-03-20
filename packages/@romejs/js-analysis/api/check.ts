/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CheckProvider} from '../types';
import {PartialDiagnostics, PartialDiagnosticAdvice} from '@romejs/diagnostics';
import {Program} from '@romejs/js-ast';
import Hub from '../Hub';
import E from '../types/errors/E';
import T from '../types/T';
import OpenT from '../types/OpenT';
import buildGraph from './buildGraph';
import {TransformProjectDefinition} from '@romejs/js-compiler';

export default async function check(
  opts: {
    ast: Program;
    project: TransformProjectDefinition;
    provider: CheckProvider;
  },
): Promise<PartialDiagnostics> {
  const hub = await buildGraph({
    ast: opts.ast,
    connected: true,
    provider: opts.provider,
    project: opts.project,
  });
  resolveGraph(hub);
  return hub.context.diagnostics;
}

function isError(t: undefined | T): boolean {
  return t !== undefined && t instanceof E;
}

function resolveGraph(hub: Hub): PartialDiagnostics {
  const {graph, utils, context} = hub;

  // we track caught errors here as if a normal type returns a error in it's reduce() method

  // then it will be added to the graph, however we'd have already dealt with it
  const caughtErrors: Set<T> = new Set();

  for (const node of graph.nodes) {
    const lower = node.value;

    // unconnected node, we'll resolve these if they've been connected to any nodes
    if (lower instanceof OpenT) {
      continue;
    }

    // see if this reduces to a type error
    const reduced = utils.reduce(lower);
    if (reduced instanceof E) {
      if (caughtErrors.has(reduced)) {
        continue;
      } else {
        caughtErrors.add(reduced);
      }

      const {
        category,
        lowerTarget,
        upperTarget,
        advice: rawAdvice,
        message,
      } = reduced.getError();

      // ignore errors inside
      if (isError(lowerTarget) || isError(upperTarget)) {
        continue;
      }

      let advice: PartialDiagnosticAdvice = [];

      if (upperTarget !== undefined) {
        const marker = upperTarget && !(upperTarget instanceof
        reduced.constructor) ? utils.humanize(upperTarget) : undefined;
        const {originLoc} = upperTarget;

        if (originLoc !== undefined && marker !== undefined) {
          advice.push({
            type: 'log',
            category: 'info',
            message: marker,
          });
        } else if (originLoc !== undefined) {
          advice.push({
            type: 'frame',
            filename: originLoc.filename,
            start: originLoc.start,
            end: originLoc.end,
            marker,
          });
        }
      }

      if (rawAdvice !== undefined) {
        advice = advice.concat(rawAdvice);
      }

      context.addNodeDiagnostic(lowerTarget.originNode, {
        category,
        message,
        advice,
        marker: lowerTarget && !(lowerTarget instanceof reduced.constructor)
          ? utils.humanize(lowerTarget) : undefined,
      });
      continue;
    }

    // ignore unconnected nodes
    if (node.lines.length === 0) {
      continue;
    }

    for (const line of node.lines) {
      const upper = line.value;
      const compatibility = utils.checkCompability(upper, lower);

      if (compatibility.type === 'incompatible') {
        // ignore associated errors, as they've already been handled
        if (isError(compatibility.lower) || isError(compatibility.upper)) {
          continue;
        }

        const advice: PartialDiagnosticAdvice = [
          {
            type: 'log',
            category: 'error',
            message: `This type is incompatible with expected type of`,
          },
        ];

        const {originLoc} = upper;
        if (originLoc === undefined) {
          advice.push({
            type: 'log',
            category: 'info',
            message: utils.humanize(upper),
          });
        } else {
          advice.push({
            type: 'frame',
            filename: originLoc.filename,
            start: originLoc.start,
            end: originLoc.end,
            marker: utils.humanize(upper),
          });
        }

        context.addNodeDiagnostic(compatibility.lower.originNode, {
          category: 'typeCheck/incompatible',
          message: 'Type incompatibility found',
          marker: utils.humanize(compatibility.lower),
          advice,
        });
      }
    }
  }

  return context.diagnostics;
}
