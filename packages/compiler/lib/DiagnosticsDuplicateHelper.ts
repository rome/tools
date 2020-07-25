/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticLocation,
	buildDuplicateLocationAdvice,
} from "@romefrontend/diagnostics";
import CompilerContext from "./CompilerContext";

type DescriptionFactory = (key: string) => DiagnosticDescription;

export class DiagnosticsDuplicateHelper {
	constructor(context: CompilerContext, descriptionFactory: DescriptionFactory) {
		this.context = context;
		this.category = descriptionFactory("").category;
		this.descriptionFactory = descriptionFactory;
		this.locations = new Map();
	}

	category: DiagnosticCategory;
	descriptionFactory: DescriptionFactory;
	locations: Map<string, Array<undefined | DiagnosticLocation>>;
	context: CompilerContext;

	addLocation(
		key: string,
		location: undefined | DiagnosticLocation,
	): {
		duplicate: boolean;
	} {
		const isSuppressed = this.context.hasLocSuppression(location, this.category);
		if (isSuppressed) {
			// If this location has had it's diagnostic suppressed then we don't want to return
			// that it was a duplicate even if there's multiple occurences
			return {duplicate: false};
		}

		let locations = this.locations.get(key);
		if (locations === undefined) {
			locations = [];
			this.locations.set(key, locations);
		}
		locations.push(location);
		return {duplicate: locations.length > 1};
	}

	process() {
		for (const [key, locations] of this.locations) {
			if (locations.length <= 1) {
				continue;
			}

			const description = this.descriptionFactory(key);
			const firstLocation = locations[0];
			const restLocations = locations.slice(1);

			this.context.addLocDiagnostic(
				firstLocation,
				{
					...description,
					advice: [
						...description.advice,
						...buildDuplicateLocationAdvice(restLocations),
					],
				},
			);
		}
	}
}
