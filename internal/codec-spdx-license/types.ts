import {SemverVersionNode} from "@internal/codec-semver";
import {CompilerProjects} from "@internal/compiler";
import {ComplexNode, ParserOptions} from "@internal/parser-core";

export type SPDXLicenseParserOptions = ParserOptions & {
	loose?: boolean;
	exceptions?: SPDXLicenseParserExceptions;
};

export type SPDXLicenseParserExceptions = {
	packageName: string;
	packageVersion: SemverVersionNode;
	projects: CompilerProjects;
};

export type SPDXLicenseDefinition = {
	reference: string;
	isDeprecatedLicenseId: boolean;
	isFsfLibre?: boolean;
	detailsUrl: string;
	referenceNumber: string;
	name: string;
	licenseId: string;
	seeAlso: string[];
	isOsiApproved: boolean;
};

export type AndNode = ComplexNode<
	"And",
	{
		left: ExpressionNode;
		right: ExpressionNode;
	}
>;

export type OrNode = ComplexNode<
	"Or",
	{
		left: ExpressionNode;
		right: ExpressionNode;
	}
>;

export type LicenseNode = ComplexNode<
	"License",
	{
		plus: boolean;
		id: string;
		exception: undefined | string;
	}
>;

//# Nodes
export type ExpressionNode = LicenseNode | AndNode | OrNode;
