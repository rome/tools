import {ARIARole} from "@romejs/compiler/lint/utils/aria/types";
import {ariaRolesMap} from "@romejs/compiler/lint/utils/aria/index";

export type MapOfElementsToConcepts = Map<string, Set<ARIARole>>;

const elementsToConceptsMap: MapOfElementsToConcepts = new Map();

for (const [, attributes] of ariaRolesMap) {
	if (attributes.baseConcepts) {
		attributes.baseConcepts.forEach(({module, concept}) => {
			if (module === "HTML") {
				if (!elementsToConceptsMap.has(concept.name)) {
					elementsToConceptsMap.set(
						concept.name,
						new Set(attributes.superClassRole),
					);
				}
			}
		});
	}
}
for (const [, attributes] of ariaRolesMap) {
	if (attributes.baseConcepts) {
		attributes.baseConcepts.forEach(({module, concept}) => {
			if (module === "HTML") {
				if (!elementsToConceptsMap.has(concept.name)) {
					elementsToConceptsMap.set(
						concept.name,
						new Set(attributes.superClassRole),
					);
				}
			}
		});
	}
}

export default elementsToConceptsMap;
