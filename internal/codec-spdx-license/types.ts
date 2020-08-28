import {ComplexNode} from "@internal/parser-core";

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
