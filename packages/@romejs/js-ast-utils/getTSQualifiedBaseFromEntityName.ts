import {AnyTSEntityName, JSReferenceIdentifier} from "@romejs/ast";

export default function getTSQualifiedBaseFromEntityName(
	entity: AnyTSEntityName,
): JSReferenceIdentifier {
	switch (entity.type) {
		case "TSQualifiedName":
			return getTSQualifiedBaseFromEntityName(entity.left);

		case "JSReferenceIdentifier":
			return entity;
	}
}
