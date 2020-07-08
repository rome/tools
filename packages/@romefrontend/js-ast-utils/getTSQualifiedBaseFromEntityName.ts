import {AnyTSEntityName, JSReferenceIdentifier} from "@romefrontend/ast";

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
