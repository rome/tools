/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  DiagnosticPointer,
  PartialDiagnostic,
  DiagnosticCategory,
} from '@romejs/diagnostics';
import Consumer from './Consumer';
import {UnknownFilePath} from '@romejs/path';
import {Number0, Number1} from '@romejs/ob1';

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
  getDiagnosticPointer?: (keys: ConsumePath, target: ConsumeSourceLocationRequestTarget) =>
      | undefined
      | DiagnosticPointer;
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
  & {type:
        | 'string'
        | 'number'
        | 'boolean'
        | 'bigint'
        | 'date'
        | 'array'
        | 'object'};

type ConsumePropertyNumberRangeDefinition =
  & ConsumePropertyDefinitionBase
  & {
    type: 'number-range';
    min: undefined | Number0 | Number1 | number;
    max: undefined | Number0 | Number1 | number;
  };

export type ConsumePropertyDefinition =
    | ConsumePropertyPrimitiveDefinition
    | ConsumePropertyNumberRangeDefinition;

export type ConsumerOnDefinition = (definition: ConsumePropertyDefinition) => void;

export type ConsumerHandleUnexpected = (diagnostic: PartialDiagnostic) => void;

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
