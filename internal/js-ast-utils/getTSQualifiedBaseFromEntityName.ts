import {AnyTSEntityName, JSReferenceIdentifier} from "@internal/ast";

export function getTSQualifiedBaseFromEntityName(
	entity: AnyTSEntityName,
): JSReferenceIdentifier {
	switch (entity.type) {
		case "TSQualifiedName":
			return getTSQualifiedBaseFromEntityName(entity.left);

		case "JSReferenceIdentifier":
			return entity;
	}
}
