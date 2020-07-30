import {ARIARole, ariaRolesMap} from "@internal/compiler/lint/utils/aria";

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
