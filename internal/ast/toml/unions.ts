import * as n from "@internal/ast";

export type AnyTomlValue =
	| n.TomlValueInteger
	| n.TomlValueString
	| n.TomlValueBoolean
	| n.TomlValueArray
	| n.TomlValueFloat
	| n.TomlValueDateTime
	| n.TomlValueInlineTable;

export type AnyTomlNode =
	| n.TomlKeyValue
		// TODO: temporary, to remove
	| n.TomlText
	| n.TomlTable;
