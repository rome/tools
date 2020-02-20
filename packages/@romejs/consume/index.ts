/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConsumerOptions, ConsumeContext} from './types';
import Consumer from './Consumer';

export const EMPTY_CONSUME_CONTEXT: ConsumeContext = {
  category: 'unknown',

  getOriginalValue() {
    return undefined;
  },

  getDiagnosticPointer() {
    return undefined;
  },
};

const EMPTY_CONSUME_OPTIONS: ConsumerOptions = {
  propertyMetadata: undefined,
  value: undefined,
  handleUnexpectedDiagnostic: undefined,
  onDefinition: undefined,
  filePath: undefined,
  context: EMPTY_CONSUME_CONTEXT,
  objectPath: [],
  parent: undefined,
};

export function consume(
  opts: Partial<Omit<ConsumerOptions, 'value' | 'context'>> & {
    value: unknown;
    context?: Partial<ConsumeContext>;
  },
): Consumer {
  return new Consumer({
    ...EMPTY_CONSUME_OPTIONS,
    ...opts,
    context: {
      ...EMPTY_CONSUME_CONTEXT,
      ...opts.context,
    },
  });
}

export function consumeUnknown(value: unknown): Consumer {
  return new Consumer({
    ...EMPTY_CONSUME_OPTIONS,
    value,
  });
}

export {Consumer};

export * from './types';
