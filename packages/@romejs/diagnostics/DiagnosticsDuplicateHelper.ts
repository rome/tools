/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import DiagnosticsProcessor from './DiagnosticsProcessor';
import {DiagnosticLocation, DiagnosticDescription} from './types';
import {buildDuplicateLocationAdvice} from './helpers';

type DescriptionFactory = (key: string) => DiagnosticDescription;

export class DiagnosticsDuplicateHelper {
  constructor(
    processor: DiagnosticsProcessor,
    descriptionFactory: DescriptionFactory,
  ) {
    this.processor = processor;
    this.descriptionFactory = descriptionFactory;
    this.locations = new Map();
  }

  descriptionFactory: DescriptionFactory;
  locations: Map<string, Array<DiagnosticLocation>>;
  processor: DiagnosticsProcessor;

  addLocation(key: string, location: undefined | DiagnosticLocation) {
    if (location === undefined) {
      // This shouldn't really happen. We'll normally be passing in `node.loc` into here
      // which can possibly be `undefined`. We don't want to error though in case it was
      // inserted dynamically as we'd have nowhere to point to.
      return;
    }

    let locations = this.locations.get(key);
    if (locations === undefined) {
      locations = [];
      this.locations.set(key, locations);
    }
    locations.push(location);
  }

  process() {
    for (const [key, locations] of this.locations) {
      if (locations.length === 1) {
        continue;
      }

      const description = this.descriptionFactory(key);
      const firstLocation = locations[0];
      const restLocations = locations.slice(1);

      this.processor.addDiagnostic({
        location: firstLocation,
        description: {
          ...description,
          advice: [
            ...(description.advice || []),
            ...buildDuplicateLocationAdvice(restLocations),
          ],
        },
      });
    }
  }
}
