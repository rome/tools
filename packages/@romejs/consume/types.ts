/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  DiagnosticLocation,
  Diagnostic,
  DiagnosticCategory,
} from '@romejs/diagnostics';
import Consumer from './Consumer';
import {UnknownFilePath} from '@romejs/path';

export type ConsumeComments = Array<string>;

export type ConsumeKey = number | string;

export type ConsumePath = Array<ConsumeKey>;

export type ConsumeSourceLocationRequestTarget =
  | 'all'
  | 'key'
  | 'value'
  | 'inner-value';

export type ConsumeContext = {
  category: DiagnosticCategory;
  normalizeKey?: (key: string) => string;
  getDiagnosticPointer?: (
    keys: ConsumePath,
    target: ConsumeSourceLocationRequestTarget,
  ) => DiagnosticLocation;
  getOriginalValue?: (path: ConsumePath) => unknown;
};

export type ConsumePropertyMetadata = {description?: string};

type ConsumePropertyDefinitionBase = {
  objectPath: ConsumePath;
  default: unknown;
  required: boolean;
  metadata?: ConsumePropertyMetadata;
};

type ConsumePropertyPrimitiveDefinition =
  & ConsumePropertyDefinitionBase
  & {
    type:
      | 'string'
      | 'number'
      | 'boolean'
      | 'bigint'
      | 'date'
      | 'array'
      | 'object';
  };

type ConsumePropertyNumberRangeDefinition = ConsumePropertyDefinitionBase & {
  type: 'number-range';
  min: undefined | number;
  max: undefined | number;
};

export type ConsumePropertyDefinition =
  | ConsumePropertyPrimitiveDefinition
  | ConsumePropertyNumberRangeDefinition;

export type ConsumerOnDefinition = (
  definition: ConsumePropertyDefinition,
  consumer: Consumer,
) => void;

export type ConsumerHandleUnexpected = (diagnostic: Diagnostic) => void;

export type ConsumerOptions = {
  handleUnexpectedDiagnostic?: ConsumerHandleUnexpected;
  onDefinition?: ConsumerOnDefinition;
  propertyMetadata?: ConsumePropertyMetadata;
  filePath?: UnknownFilePath;
  objectPath: ConsumePath;
  context: ConsumeContext;
  value: unknown;
  parent?: Consumer;
  forceDiagnosticTarget?: ConsumeSourceLocationRequestTarget;
};
