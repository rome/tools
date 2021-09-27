use crate::{
	ast::{support, AstChildren, AstNode},
	SyntaxKind, SyntaxNode, SyntaxToken,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractClassDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl AbstractClassDeclaration {
	pub fn body(&self) -> Option<ClassBody> {
		support::child(&self.syntax)
	}
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractMethodSignature {
	pub(crate) syntax: SyntaxNode,
}
impl AbstractMethodSignature {
	pub fn name(&self) -> Option<AbstractMethodSignatureName> {
		support::child(&self.syntax)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<AbstractMethodSignatureReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessibilityModifier {
	pub(crate) syntax: SyntaxNode,
}
impl AccessibilityModifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmbientDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl AmbientDeclaration {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arguments {
	pub(crate) syntax: SyntaxNode,
}
impl Arguments {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
	pub(crate) syntax: SyntaxNode,
}
impl Array {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ArrayPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
	pub(crate) syntax: SyntaxNode,
}
impl ArrayType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrowFunction {
	pub(crate) syntax: SyntaxNode,
}
impl ArrowFunction {
	pub fn body(&self) -> Option<ArrowFunctionBody> {
		support::child(&self.syntax)
	}
	pub fn parameter(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<ArrowFunctionReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsExpression {
	pub(crate) syntax: SyntaxNode,
}
impl AsExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asserts {
	pub(crate) syntax: SyntaxNode,
}
impl Asserts {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignmentExpression {
	pub(crate) syntax: SyntaxNode,
}
impl AssignmentExpression {
	pub fn left(&self) -> Option<AssignmentExpressionLeft> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl AssignmentPattern {
	pub fn left(&self) -> Option<Pattern> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AugmentedAssignmentExpression {
	pub(crate) syntax: SyntaxNode,
}
impl AugmentedAssignmentExpression {
	pub fn left(&self) -> Option<AugmentedAssignmentExpressionLeft> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitExpression {
	pub(crate) syntax: SyntaxNode,
}
impl AwaitExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl BinaryExpression {
	pub fn left(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn operator(&self) -> Option<BinaryExpressionOperator> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakStatement {
	pub(crate) syntax: SyntaxNode,
}
impl BreakStatement {
	pub fn label(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::StatementIdentifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpression {
	pub(crate) syntax: SyntaxNode,
}
impl CallExpression {
	pub fn arguments(&self) -> Option<CallExpressionArguments> {
		support::child(&self.syntax)
	}
	pub fn function(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn type_arguments(&self) -> Option<TypeArguments> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallSignature {
	pub(crate) syntax: SyntaxNode,
}
impl CallSignature {
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<CallSignatureReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchClause {
	pub(crate) syntax: SyntaxNode,
}
impl CatchClause {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn parameter(&self) -> Option<CatchClauseParameter> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
	pub(crate) syntax: SyntaxNode,
}
impl Class {
	pub fn body(&self) -> Option<ClassBody> {
		support::child(&self.syntax)
	}
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassBody {
	pub(crate) syntax: SyntaxNode,
}
impl ClassBody {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl ClassDeclaration {
	pub fn body(&self) -> Option<ClassBody> {
		support::child(&self.syntax)
	}
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassHeritage {
	pub(crate) syntax: SyntaxNode,
}
impl ClassHeritage {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComputedPropertyName {
	pub(crate) syntax: SyntaxNode,
}
impl ComputedPropertyName {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalType {
	pub(crate) syntax: SyntaxNode,
}
impl ConditionalType {
	pub fn alternative(&self) -> Option<ConditionalTypeAlternative> {
		support::child(&self.syntax)
	}
	pub fn consequence(&self) -> Option<ConditionalTypeConsequence> {
		support::child(&self.syntax)
	}
	pub fn left(&self) -> Option<ConditionalTypeLeft> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<ConditionalTypeRight> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraint {
	pub(crate) syntax: SyntaxNode,
}
impl Constraint {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructSignature {
	pub(crate) syntax: SyntaxNode,
}
impl ConstructSignature {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorType {
	pub(crate) syntax: SyntaxNode,
}
impl ConstructorType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ContinueStatement {
	pub fn label(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::StatementIdentifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebuggerStatement {
	pub(crate) syntax: SyntaxNode,
}
impl DebuggerStatement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decorator {
	pub(crate) syntax: SyntaxNode,
}
impl Decorator {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultType {
	pub(crate) syntax: SyntaxNode,
}
impl DefaultType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoStatement {
	pub(crate) syntax: SyntaxNode,
}
impl DoStatement {
	pub fn body(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
	pub fn condition(&self) -> Option<ParenthesizedExpression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseClause {
	pub(crate) syntax: SyntaxNode,
}
impl ElseClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmptyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl EmptyStatement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl EnumAssignment {
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumBody {
	pub(crate) syntax: SyntaxNode,
}
impl EnumBody {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl EnumDeclaration {
	pub fn body(&self) -> Option<EnumBody> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExistentialType {
	pub(crate) syntax: SyntaxNode,
}
impl ExistentialType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportClause {
	pub(crate) syntax: SyntaxNode,
}
impl ExportClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl ExportSpecifier {
	pub fn alias(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ExportStatement {
	pub fn declaration(&self) -> Option<Declaration> {
		support::child(&self.syntax)
	}
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn source(&self) -> Option<StringLiteral> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ExpressionStatement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtendsClause {
	pub(crate) syntax: SyntaxNode,
}
impl ExtendsClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FinallyClause {
	pub(crate) syntax: SyntaxNode,
}
impl FinallyClause {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowMaybeType {
	pub(crate) syntax: SyntaxNode,
}
impl FlowMaybeType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForInStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ForInStatement {
	pub fn body(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
	pub fn left(&self) -> Option<ForInStatementLeft> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<ForInStatementRight> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ForStatement {
	pub fn body(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
	pub fn condition(&self) -> Option<ForStatementCondition> {
		support::child(&self.syntax)
	}
	pub fn increment(&self) -> Option<ForStatementIncrement> {
		support::child(&self.syntax)
	}
	pub fn initializer(&self) -> Option<ForStatementInitializer> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormalParameters {
	pub(crate) syntax: SyntaxNode,
}
impl FormalParameters {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
	pub(crate) syntax: SyntaxNode,
}
impl Function {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<FunctionReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl FunctionDeclaration {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<FunctionDeclarationReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
	pub(crate) syntax: SyntaxNode,
}
impl FunctionSignature {
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<FunctionSignatureReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
	pub(crate) syntax: SyntaxNode,
}
impl FunctionType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratorFunction {
	pub(crate) syntax: SyntaxNode,
}
impl GeneratorFunction {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<GeneratorFunctionReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratorFunctionDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl GeneratorFunctionDeclaration {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<GeneratorFunctionDeclarationReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericType {
	pub(crate) syntax: SyntaxNode,
}
impl GenericType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl IfStatement {
	pub fn alternative(&self) -> Option<ElseClause> {
		support::child(&self.syntax)
	}
	pub fn condition(&self) -> Option<ParenthesizedExpression> {
		support::child(&self.syntax)
	}
	pub fn consequence(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementsClause {
	pub(crate) syntax: SyntaxNode,
}
impl ImplementsClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Import {
	pub(crate) syntax: SyntaxNode,
}
impl Import {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportAlias {
	pub(crate) syntax: SyntaxNode,
}
impl ImportAlias {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportClause {
	pub(crate) syntax: SyntaxNode,
}
impl ImportClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportRequireClause {
	pub(crate) syntax: SyntaxNode,
}
impl ImportRequireClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl ImportSpecifier {
	pub fn alias(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::Identifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ImportStatement {
	pub fn source(&self) -> Option<StringLiteral> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexSignature {
	pub(crate) syntax: SyntaxNode,
}
impl IndexSignature {
	pub fn sign(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::DashToken)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexTypeQuery {
	pub(crate) syntax: SyntaxNode,
}
impl IndexTypeQuery {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InferType {
	pub(crate) syntax: SyntaxNode,
}
impl InferType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl InterfaceDeclaration {
	pub fn body(&self) -> Option<ObjectType> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InternalModule {
	pub(crate) syntax: SyntaxNode,
}
impl InternalModule {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<InternalModuleName> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntersectionType {
	pub(crate) syntax: SyntaxNode,
}
impl IntersectionType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxAttribute {
	pub(crate) syntax: SyntaxNode,
}
impl JsxAttribute {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxClosingElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsxClosingElement {
	pub fn name(&self) -> Option<JsxClosingElementName> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsxElement {
	pub fn close_tag(&self) -> Option<JsxClosingElement> {
		support::child(&self.syntax)
	}
	pub fn open_tag(&self) -> Option<JsxOpeningElement> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsxExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxFragment {
	pub(crate) syntax: SyntaxNode,
}
impl JsxFragment {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxNamespaceName {
	pub(crate) syntax: SyntaxNode,
}
impl JsxNamespaceName {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxOpeningElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsxOpeningElement {
	pub fn attribute(&self) -> AstChildren<JsxOpeningElementAttribute> {
		support::children(&self.syntax)
	}
	pub fn name(&self) -> Option<JsxOpeningElementName> {
		support::child(&self.syntax)
	}
	pub fn type_arguments(&self) -> Option<TypeArguments> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxSelfClosingElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsxSelfClosingElement {
	pub fn attribute(&self) -> AstChildren<JsxSelfClosingElementAttribute> {
		support::children(&self.syntax)
	}
	pub fn name(&self) -> Option<JsxSelfClosingElementName> {
		support::child(&self.syntax)
	}
	pub fn type_arguments(&self) -> Option<TypeArguments> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabeledStatement {
	pub(crate) syntax: SyntaxNode,
}
impl LabeledStatement {
	pub fn label(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::StatementIdentifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LexicalDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl LexicalDeclaration {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralType {
	pub(crate) syntax: SyntaxNode,
}
impl LiteralType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookupType {
	pub(crate) syntax: SyntaxNode,
}
impl LookupType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MappedTypeClause {
	pub(crate) syntax: SyntaxNode,
}
impl MappedTypeClause {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl MemberExpression {
	pub fn object(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn property(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::PropertyIdentifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaProperty {
	pub(crate) syntax: SyntaxNode,
}
impl MetaProperty {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodDefinition {
	pub(crate) syntax: SyntaxNode,
}
impl MethodDefinition {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<MethodDefinitionName> {
		support::child(&self.syntax)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<MethodDefinitionReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodSignature {
	pub(crate) syntax: SyntaxNode,
}
impl MethodSignature {
	pub fn name(&self) -> Option<MethodSignatureName> {
		support::child(&self.syntax)
	}
	pub fn parameters(&self) -> Option<FormalParameters> {
		support::child(&self.syntax)
	}
	pub fn return_type(&self) -> Option<MethodSignatureReturnType> {
		support::child(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
	pub(crate) syntax: SyntaxNode,
}
impl Module {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<ModuleName> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedImports {
	pub(crate) syntax: SyntaxNode,
}
impl NamedImports {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamespaceImport {
	pub(crate) syntax: SyntaxNode,
}
impl NamespaceImport {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NestedIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl NestedIdentifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NestedTypeIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl NestedTypeIdentifier {
	pub fn module(&self) -> Option<NestedTypeIdentifierModule> {
		support::child(&self.syntax)
	}
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewExpression {
	pub(crate) syntax: SyntaxNode,
}
impl NewExpression {
	pub fn arguments(&self) -> Option<Arguments> {
		support::child(&self.syntax)
	}
	pub fn constructor(&self) -> Option<PrimaryExpression> {
		support::child(&self.syntax)
	}
	pub fn type_arguments(&self) -> Option<TypeArguments> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonNullExpression {
	pub(crate) syntax: SyntaxNode,
}
impl NonNullExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Object {
	pub(crate) syntax: SyntaxNode,
}
impl Object {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectAssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectAssignmentPattern {
	pub fn left(&self) -> Option<ObjectAssignmentPatternLeft> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectType {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OmittingTypeAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl OmittingTypeAnnotation {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptingTypeAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl OptingTypeAnnotation {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionalParameter {
	pub(crate) syntax: SyntaxNode,
}
impl OptionalParameter {
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionalType {
	pub(crate) syntax: SyntaxNode,
}
impl OptionalType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pair {
	pub(crate) syntax: SyntaxNode,
}
impl Pair {
	pub fn key(&self) -> Option<PairKey> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairPattern {
	pub(crate) syntax: SyntaxNode,
}
impl PairPattern {
	pub fn key(&self) -> Option<PairPatternKey> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<Pattern> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenthesizedExpression {
	pub(crate) syntax: SyntaxNode,
}
impl ParenthesizedExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenthesizedType {
	pub(crate) syntax: SyntaxNode,
}
impl ParenthesizedType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredefinedType {
	pub(crate) syntax: SyntaxNode,
}
impl PredefinedType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
	pub(crate) syntax: SyntaxNode,
}
impl Program {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertySignature {
	pub(crate) syntax: SyntaxNode,
}
impl PropertySignature {
	pub fn name(&self) -> Option<PropertySignatureName> {
		support::child(&self.syntax)
	}
	pub fn ty(&self) -> Option<TypeAnnotation> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicFieldDefinition {
	pub(crate) syntax: SyntaxNode,
}
impl PublicFieldDefinition {
	pub fn name(&self) -> Option<PublicFieldDefinitionName> {
		support::child(&self.syntax)
	}
	pub fn ty(&self) -> Option<TypeAnnotation> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadonlyType {
	pub(crate) syntax: SyntaxNode,
}
impl ReadonlyType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Regex {
	pub(crate) syntax: SyntaxNode,
}
impl Regex {
	pub fn flags(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::RegexFlags)
	}
	pub fn pattern(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::RegexPattern)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequiredParameter {
	pub(crate) syntax: SyntaxNode,
}
impl RequiredParameter {
	pub fn decorator(&self) -> AstChildren<Decorator> {
		support::children(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestPattern {
	pub(crate) syntax: SyntaxNode,
}
impl RestPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestType {
	pub(crate) syntax: SyntaxNode,
}
impl RestType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ReturnStatement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceExpression {
	pub(crate) syntax: SyntaxNode,
}
impl SequenceExpression {
	pub fn left(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn right(&self) -> Option<SequenceExpressionRight> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpreadElement {
	pub(crate) syntax: SyntaxNode,
}
impl SpreadElement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatementBlock {
	pub(crate) syntax: SyntaxNode,
}
impl StatementBlock {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl StringLiteral {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptExpression {
	pub(crate) syntax: SyntaxNode,
}
impl SubscriptExpression {
	pub fn index(&self) -> Option<SubscriptExpressionIndex> {
		support::child(&self.syntax)
	}
	pub fn object(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchBody {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchBody {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchCase {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchCase {
	pub fn value(&self) -> Option<SwitchCaseValue> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchDefault {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchDefault {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStatement {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchStatement {
	pub fn body(&self) -> Option<SwitchBody> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<ParenthesizedExpression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateString {
	pub(crate) syntax: SyntaxNode,
}
impl TemplateString {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateSubstitution {
	pub(crate) syntax: SyntaxNode,
}
impl TemplateSubstitution {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TernaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl TernaryExpression {
	pub fn alternative(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn condition(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn consequence(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStatement {
	pub(crate) syntax: SyntaxNode,
}
impl ThrowStatement {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryStatement {
	pub(crate) syntax: SyntaxNode,
}
impl TryStatement {
	pub fn body(&self) -> Option<StatementBlock> {
		support::child(&self.syntax)
	}
	pub fn finalizer(&self) -> Option<FinallyClause> {
		support::child(&self.syntax)
	}
	pub fn handler(&self) -> Option<CatchClause> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
	pub(crate) syntax: SyntaxNode,
}
impl TupleType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAliasDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl TypeAliasDeclaration {
	pub fn name(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::TypeIdentifier)
	}
	pub fn type_parameters(&self) -> Option<TypeParameters> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<TypeAliasDeclarationValue> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl TypeAnnotation {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArguments {
	pub(crate) syntax: SyntaxNode,
}
impl TypeArguments {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
	pub(crate) syntax: SyntaxNode,
}
impl TypeParameter {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameters {
	pub(crate) syntax: SyntaxNode,
}
impl TypeParameters {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePredicate {
	pub(crate) syntax: SyntaxNode,
}
impl TypePredicate {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePredicateAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl TypePredicateAnnotation {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeQuery {
	pub(crate) syntax: SyntaxNode,
}
impl TypeQuery {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl UnaryExpression {
	pub fn argument(&self) -> Option<UnaryExpressionArgument> {
		support::child(&self.syntax)
	}
	pub fn operator(&self) -> Option<UnaryExpressionOperator> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnionType {
	pub(crate) syntax: SyntaxNode,
}
impl UnionType {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl UpdateExpression {
	pub fn argument(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
	pub fn operator(&self) -> Option<UpdateExpressionOperator> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl VariableDeclaration {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDeclarator {
	pub(crate) syntax: SyntaxNode,
}
impl VariableDeclarator {
	pub fn name(&self) -> Option<VariableDeclaratorName> {
		support::child(&self.syntax)
	}
	pub fn ty(&self) -> Option<TypeAnnotation> {
		support::child(&self.syntax)
	}
	pub fn value(&self) -> Option<Expression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl WhileStatement {
	pub fn body(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
	pub fn condition(&self) -> Option<ParenthesizedExpression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithStatement {
	pub(crate) syntax: SyntaxNode,
}
impl WithStatement {
	pub fn body(&self) -> Option<Statement> {
		support::child(&self.syntax)
	}
	pub fn object(&self) -> Option<ParenthesizedExpression> {
		support::child(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YieldExpression {
	pub(crate) syntax: SyntaxNode,
}
impl YieldExpression {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BangToken {
	pub(crate) syntax: SyntaxNode,
}
impl BangToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BangEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl BangEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BangEqEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl BangEqEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DquoteToken {
	pub(crate) syntax: SyntaxNode,
}
impl DquoteToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DollarLbraceToken {
	pub(crate) syntax: SyntaxNode,
}
impl DollarLbraceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentToken {
	pub(crate) syntax: SyntaxNode,
}
impl PercentToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl PercentEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpToken {
	pub(crate) syntax: SyntaxNode,
}
impl AmpToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpAmpToken {
	pub(crate) syntax: SyntaxNode,
}
impl AmpAmpToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpAmpEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl AmpAmpEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl AmpEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SquoteToken {
	pub(crate) syntax: SyntaxNode,
}
impl SquoteToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LparenToken {
	pub(crate) syntax: SyntaxNode,
}
impl LparenToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RparenToken {
	pub(crate) syntax: SyntaxNode,
}
impl RparenToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarToken {
	pub(crate) syntax: SyntaxNode,
}
impl StarToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarStarToken {
	pub(crate) syntax: SyntaxNode,
}
impl StarStarToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarStarEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl StarStarEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl StarEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusToken {
	pub(crate) syntax: SyntaxNode,
}
impl PlusToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusPlusToken {
	pub(crate) syntax: SyntaxNode,
}
impl PlusPlusToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl PlusEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommaToken {
	pub(crate) syntax: SyntaxNode,
}
impl CommaToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DashToken {
	pub(crate) syntax: SyntaxNode,
}
impl DashToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DashDashToken {
	pub(crate) syntax: SyntaxNode,
}
impl DashDashToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DashEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl DashEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DashQmarkColonToken {
	pub(crate) syntax: SyntaxNode,
}
impl DashQmarkColonToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotToken {
	pub(crate) syntax: SyntaxNode,
}
impl DotToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotDotDotToken {
	pub(crate) syntax: SyntaxNode,
}
impl DotDotDotToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlashToken {
	pub(crate) syntax: SyntaxNode,
}
impl SlashToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlashEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl SlashEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColonToken {
	pub(crate) syntax: SyntaxNode,
}
impl ColonToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemiToken {
	pub(crate) syntax: SyntaxNode,
}
impl SemiToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtToken {
	pub(crate) syntax: SyntaxNode,
}
impl LtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtLtToken {
	pub(crate) syntax: SyntaxNode,
}
impl LtLtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtLtEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl LtLtEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl LtEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqToken {
	pub(crate) syntax: SyntaxNode,
}
impl EqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl EqEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqEqEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl EqEqEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqGtToken {
	pub(crate) syntax: SyntaxNode,
}
impl EqGtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtGtToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtGtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtGtEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtGtEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtGtGtToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtGtGtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtGtGtEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl GtGtGtEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QmarkToken {
	pub(crate) syntax: SyntaxNode,
}
impl QmarkToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QmarkDotToken {
	pub(crate) syntax: SyntaxNode,
}
impl QmarkDotToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QmarkColonToken {
	pub(crate) syntax: SyntaxNode,
}
impl QmarkColonToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QmarkQmarkToken {
	pub(crate) syntax: SyntaxNode,
}
impl QmarkQmarkToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QmarkQmarkEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl QmarkQmarkEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtToken {
	pub(crate) syntax: SyntaxNode,
}
impl AtToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LbrackToken {
	pub(crate) syntax: SyntaxNode,
}
impl LbrackToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RbrackToken {
	pub(crate) syntax: SyntaxNode,
}
impl RbrackToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaretToken {
	pub(crate) syntax: SyntaxNode,
}
impl CaretToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaretEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl CaretEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BquoteToken {
	pub(crate) syntax: SyntaxNode,
}
impl BquoteToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractToken {
	pub(crate) syntax: SyntaxNode,
}
impl AbstractToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyToken {
	pub(crate) syntax: SyntaxNode,
}
impl AnyToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsToken {
	pub(crate) syntax: SyntaxNode,
}
impl AsToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssertsToken {
	pub(crate) syntax: SyntaxNode,
}
impl AssertsToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncToken {
	pub(crate) syntax: SyntaxNode,
}
impl AsyncToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitToken {
	pub(crate) syntax: SyntaxNode,
}
impl AwaitToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BooleanToken {
	pub(crate) syntax: SyntaxNode,
}
impl BooleanToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakToken {
	pub(crate) syntax: SyntaxNode,
}
impl BreakToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseToken {
	pub(crate) syntax: SyntaxNode,
}
impl CaseToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchToken {
	pub(crate) syntax: SyntaxNode,
}
impl CatchToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassToken {
	pub(crate) syntax: SyntaxNode,
}
impl ClassToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
	pub(crate) syntax: SyntaxNode,
}
impl Comment {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstToken {
	pub(crate) syntax: SyntaxNode,
}
impl ConstToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueToken {
	pub(crate) syntax: SyntaxNode,
}
impl ContinueToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebuggerToken {
	pub(crate) syntax: SyntaxNode,
}
impl DebuggerToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclareToken {
	pub(crate) syntax: SyntaxNode,
}
impl DeclareToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultToken {
	pub(crate) syntax: SyntaxNode,
}
impl DefaultToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeleteToken {
	pub(crate) syntax: SyntaxNode,
}
impl DeleteToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoToken {
	pub(crate) syntax: SyntaxNode,
}
impl DoToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseToken {
	pub(crate) syntax: SyntaxNode,
}
impl ElseToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumToken {
	pub(crate) syntax: SyntaxNode,
}
impl EnumToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EscapeSequence {
	pub(crate) syntax: SyntaxNode,
}
impl EscapeSequence {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportToken {
	pub(crate) syntax: SyntaxNode,
}
impl ExportToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtendsToken {
	pub(crate) syntax: SyntaxNode,
}
impl ExtendsToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct False {
	pub(crate) syntax: SyntaxNode,
}
impl False {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FinallyToken {
	pub(crate) syntax: SyntaxNode,
}
impl FinallyToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForToken {
	pub(crate) syntax: SyntaxNode,
}
impl ForToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromToken {
	pub(crate) syntax: SyntaxNode,
}
impl FromToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionToken {
	pub(crate) syntax: SyntaxNode,
}
impl FunctionToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetToken {
	pub(crate) syntax: SyntaxNode,
}
impl GetToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalToken {
	pub(crate) syntax: SyntaxNode,
}
impl GlobalToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashBangLine {
	pub(crate) syntax: SyntaxNode,
}
impl HashBangLine {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
	pub(crate) syntax: SyntaxNode,
}
impl Identifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfToken {
	pub(crate) syntax: SyntaxNode,
}
impl IfToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementsToken {
	pub(crate) syntax: SyntaxNode,
}
impl ImplementsToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportToken {
	pub(crate) syntax: SyntaxNode,
}
impl ImportToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InToken {
	pub(crate) syntax: SyntaxNode,
}
impl InToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InferToken {
	pub(crate) syntax: SyntaxNode,
}
impl InferToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceofToken {
	pub(crate) syntax: SyntaxNode,
}
impl InstanceofToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceToken {
	pub(crate) syntax: SyntaxNode,
}
impl InterfaceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsToken {
	pub(crate) syntax: SyntaxNode,
}
impl IsToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsxText {
	pub(crate) syntax: SyntaxNode,
}
impl JsxText {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyofToken {
	pub(crate) syntax: SyntaxNode,
}
impl KeyofToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetToken {
	pub(crate) syntax: SyntaxNode,
}
impl LetToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleToken {
	pub(crate) syntax: SyntaxNode,
}
impl ModuleToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamespaceToken {
	pub(crate) syntax: SyntaxNode,
}
impl NamespaceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewToken {
	pub(crate) syntax: SyntaxNode,
}
impl NewToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null {
	pub(crate) syntax: SyntaxNode,
}
impl Null {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number {
	pub(crate) syntax: SyntaxNode,
}
impl Number {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberToken {
	pub(crate) syntax: SyntaxNode,
}
impl NumberToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OfToken {
	pub(crate) syntax: SyntaxNode,
}
impl OfToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateToken {
	pub(crate) syntax: SyntaxNode,
}
impl PrivateToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl PropertyIdentifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtectedToken {
	pub(crate) syntax: SyntaxNode,
}
impl ProtectedToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicToken {
	pub(crate) syntax: SyntaxNode,
}
impl PublicToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadonlyToken {
	pub(crate) syntax: SyntaxNode,
}
impl ReadonlyToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexFlags {
	pub(crate) syntax: SyntaxNode,
}
impl RegexFlags {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexPattern {
	pub(crate) syntax: SyntaxNode,
}
impl RegexPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequireToken {
	pub(crate) syntax: SyntaxNode,
}
impl RequireToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnToken {
	pub(crate) syntax: SyntaxNode,
}
impl ReturnToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetToken {
	pub(crate) syntax: SyntaxNode,
}
impl SetToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShorthandPropertyIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl ShorthandPropertyIdentifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShorthandPropertyIdentifierPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ShorthandPropertyIdentifierPattern {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatementIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl StatementIdentifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticToken {
	pub(crate) syntax: SyntaxNode,
}
impl StaticToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringToken {
	pub(crate) syntax: SyntaxNode,
}
impl StringToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Super {
	pub(crate) syntax: SyntaxNode,
}
impl Super {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchToken {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolToken {
	pub(crate) syntax: SyntaxNode,
}
impl SymbolToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetToken {
	pub(crate) syntax: SyntaxNode,
}
impl TargetToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct This {
	pub(crate) syntax: SyntaxNode,
}
impl This {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowToken {
	pub(crate) syntax: SyntaxNode,
}
impl ThrowToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct True {
	pub(crate) syntax: SyntaxNode,
}
impl True {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryToken {
	pub(crate) syntax: SyntaxNode,
}
impl TryToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeToken {
	pub(crate) syntax: SyntaxNode,
}
impl TypeToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl TypeIdentifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeofToken {
	pub(crate) syntax: SyntaxNode,
}
impl TypeofToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Undefined {
	pub(crate) syntax: SyntaxNode,
}
impl Undefined {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarToken {
	pub(crate) syntax: SyntaxNode,
}
impl VarToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoidToken {
	pub(crate) syntax: SyntaxNode,
}
impl VoidToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileToken {
	pub(crate) syntax: SyntaxNode,
}
impl WhileToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithToken {
	pub(crate) syntax: SyntaxNode,
}
impl WithToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YieldToken {
	pub(crate) syntax: SyntaxNode,
}
impl YieldToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LbraceToken {
	pub(crate) syntax: SyntaxNode,
}
impl LbraceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LbracePipeToken {
	pub(crate) syntax: SyntaxNode,
}
impl LbracePipeToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeToken {
	pub(crate) syntax: SyntaxNode,
}
impl PipeToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl PipeEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipePipeToken {
	pub(crate) syntax: SyntaxNode,
}
impl PipePipeToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipePipeEqToken {
	pub(crate) syntax: SyntaxNode,
}
impl PipePipeEqToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeRbraceToken {
	pub(crate) syntax: SyntaxNode,
}
impl PipeRbraceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RbraceToken {
	pub(crate) syntax: SyntaxNode,
}
impl RbraceToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TildeToken {
	pub(crate) syntax: SyntaxNode,
}
impl TildeToken {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimaryType {
	ArrayType(ArrayType),
	ConditionalType(ConditionalType),
	ExistentialType(ExistentialType),
	FlowMaybeType(FlowMaybeType),
	GenericType(GenericType),
	IndexTypeQuery(IndexTypeQuery),
	LiteralType(LiteralType),
	LookupType(LookupType),
	NestedTypeIdentifier(NestedTypeIdentifier),
	ObjectType(ObjectType),
	ParenthesizedType(ParenthesizedType),
	PredefinedType(PredefinedType),
	This(This),
	TupleType(TupleType),
	TypeIdentifier(TypeIdentifier),
	TypeQuery(TypeQuery),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Declaration {
	AbstractClassDeclaration(AbstractClassDeclaration),
	AmbientDeclaration(AmbientDeclaration),
	ClassDeclaration(ClassDeclaration),
	EnumDeclaration(EnumDeclaration),
	FunctionDeclaration(FunctionDeclaration),
	FunctionSignature(FunctionSignature),
	GeneratorFunctionDeclaration(GeneratorFunctionDeclaration),
	ImportAlias(ImportAlias),
	InterfaceDeclaration(InterfaceDeclaration),
	InternalModule(InternalModule),
	LexicalDeclaration(LexicalDeclaration),
	Module(Module),
	TypeAliasDeclaration(TypeAliasDeclaration),
	VariableDeclaration(VariableDeclaration),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
	AsExpression(AsExpression),
	AssignmentExpression(AssignmentExpression),
	AugmentedAssignmentExpression(AugmentedAssignmentExpression),
	AwaitExpression(AwaitExpression),
	BinaryExpression(BinaryExpression),
	InternalModule(InternalModule),
	JsxElement(JsxElement),
	JsxFragment(JsxFragment),
	JsxSelfClosingElement(JsxSelfClosingElement),
	NewExpression(NewExpression),
	PrimaryExpression(PrimaryExpression),
	TernaryExpression(TernaryExpression),
	UnaryExpression(UnaryExpression),
	UpdateExpression(UpdateExpression),
	YieldExpression(YieldExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
	ArrayPattern(ArrayPattern),
	Identifier(Identifier),
	ObjectPattern(ObjectPattern),
	RestPattern(RestPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimaryExpression {
	Array(Array),
	ArrowFunction(ArrowFunction),
	CallExpression(CallExpression),
	Class(Class),
	False(False),
	Function(Function),
	GeneratorFunction(GeneratorFunction),
	Identifier(Identifier),
	Import(Import),
	MemberExpression(MemberExpression),
	MetaProperty(MetaProperty),
	NonNullExpression(NonNullExpression),
	Null(Null),
	Number(Number),
	Object(Object),
	ParenthesizedExpression(ParenthesizedExpression),
	Regex(Regex),
	StringLiteral(StringLiteral),
	SubscriptExpression(SubscriptExpression),
	Super(Super),
	TemplateString(TemplateString),
	This(This),
	True(True),
	Undefined(Undefined),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
	BreakStatement(BreakStatement),
	ContinueStatement(ContinueStatement),
	DebuggerStatement(DebuggerStatement),
	Declaration(Declaration),
	DoStatement(DoStatement),
	EmptyStatement(EmptyStatement),
	ExportStatement(ExportStatement),
	ExpressionStatement(ExpressionStatement),
	ForInStatement(ForInStatement),
	ForStatement(ForStatement),
	IfStatement(IfStatement),
	ImportStatement(ImportStatement),
	LabeledStatement(LabeledStatement),
	ReturnStatement(ReturnStatement),
	StatementBlock(StatementBlock),
	SwitchStatement(SwitchStatement),
	ThrowStatement(ThrowStatement),
	TryStatement(TryStatement),
	WhileStatement(WhileStatement),
	WithStatement(WithStatement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbstractMethodSignatureName {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbstractMethodSignatureReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrowFunctionBody {
	Expression(Expression),
	StatementBlock(StatementBlock),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrowFunctionReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignmentExpressionLeft {
	ArrayPattern(ArrayPattern),
	Identifier(Identifier),
	MemberExpression(MemberExpression),
	NonNullExpression(NonNullExpression),
	ObjectPattern(ObjectPattern),
	ParenthesizedExpression(ParenthesizedExpression),
	SubscriptExpression(SubscriptExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AugmentedAssignmentExpressionLeft {
	Identifier(Identifier),
	MemberExpression(MemberExpression),
	NonNullExpression(NonNullExpression),
	ParenthesizedExpression(ParenthesizedExpression),
	SubscriptExpression(SubscriptExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryExpressionOperator {
	BangEqToken(BangEqToken),
	BangEqEqToken(BangEqEqToken),
	PercentToken(PercentToken),
	AmpToken(AmpToken),
	AmpAmpToken(AmpAmpToken),
	StarToken(StarToken),
	StarStarToken(StarStarToken),
	PlusToken(PlusToken),
	DashToken(DashToken),
	SlashToken(SlashToken),
	LtToken(LtToken),
	LtLtToken(LtLtToken),
	LtEqToken(LtEqToken),
	EqEqToken(EqEqToken),
	EqEqEqToken(EqEqEqToken),
	GtToken(GtToken),
	GtEqToken(GtEqToken),
	GtGtToken(GtGtToken),
	GtGtGtToken(GtGtGtToken),
	QmarkQmarkToken(QmarkQmarkToken),
	CaretToken(CaretToken),
	InToken(InToken),
	InstanceofToken(InstanceofToken),
	PipeToken(PipeToken),
	PipePipeToken(PipePipeToken),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallExpressionArguments {
	Arguments(Arguments),
	TemplateString(TemplateString),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallSignatureReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CatchClauseParameter {
	ArrayPattern(ArrayPattern),
	Identifier(Identifier),
	ObjectPattern(ObjectPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionalTypeAlternative {
	PrimaryType(PrimaryType),
	ConstructorType(ConstructorType),
	FunctionType(FunctionType),
	InferType(InferType),
	IntersectionType(IntersectionType),
	ReadonlyType(ReadonlyType),
	UnionType(UnionType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionalTypeConsequence {
	PrimaryType(PrimaryType),
	ConstructorType(ConstructorType),
	FunctionType(FunctionType),
	InferType(InferType),
	IntersectionType(IntersectionType),
	ReadonlyType(ReadonlyType),
	UnionType(UnionType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionalTypeLeft {
	PrimaryType(PrimaryType),
	ConstructorType(ConstructorType),
	FunctionType(FunctionType),
	InferType(InferType),
	IntersectionType(IntersectionType),
	ReadonlyType(ReadonlyType),
	UnionType(UnionType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionalTypeRight {
	PrimaryType(PrimaryType),
	ConstructorType(ConstructorType),
	FunctionType(FunctionType),
	InferType(InferType),
	IntersectionType(IntersectionType),
	ReadonlyType(ReadonlyType),
	UnionType(UnionType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForInStatementLeft {
	ArrayPattern(ArrayPattern),
	Identifier(Identifier),
	MemberExpression(MemberExpression),
	NonNullExpression(NonNullExpression),
	ObjectPattern(ObjectPattern),
	ParenthesizedExpression(ParenthesizedExpression),
	SubscriptExpression(SubscriptExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForInStatementRight {
	Expression(Expression),
	SequenceExpression(SequenceExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForStatementCondition {
	EmptyStatement(EmptyStatement),
	ExpressionStatement(ExpressionStatement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForStatementIncrement {
	Expression(Expression),
	SequenceExpression(SequenceExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForStatementInitializer {
	EmptyStatement(EmptyStatement),
	ExpressionStatement(ExpressionStatement),
	LexicalDeclaration(LexicalDeclaration),
	VariableDeclaration(VariableDeclaration),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionDeclarationReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionSignatureReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorFunctionReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorFunctionDeclarationReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InternalModuleName {
	Identifier(Identifier),
	NestedIdentifier(NestedIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsxClosingElementName {
	Identifier(Identifier),
	JsxNamespaceName(JsxNamespaceName),
	NestedIdentifier(NestedIdentifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsxOpeningElementAttribute {
	JsxAttribute(JsxAttribute),
	JsxExpression(JsxExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsxOpeningElementName {
	Identifier(Identifier),
	JsxNamespaceName(JsxNamespaceName),
	NestedIdentifier(NestedIdentifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsxSelfClosingElementAttribute {
	JsxAttribute(JsxAttribute),
	JsxExpression(JsxExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsxSelfClosingElementName {
	Identifier(Identifier),
	JsxNamespaceName(JsxNamespaceName),
	NestedIdentifier(NestedIdentifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodDefinitionName {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodDefinitionReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodSignatureName {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodSignatureReturnType {
	Asserts(Asserts),
	TypeAnnotation(TypeAnnotation),
	TypePredicateAnnotation(TypePredicateAnnotation),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleName {
	Identifier(Identifier),
	NestedIdentifier(NestedIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NestedTypeIdentifierModule {
	Identifier(Identifier),
	NestedIdentifier(NestedIdentifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectAssignmentPatternLeft {
	ArrayPattern(ArrayPattern),
	ObjectPattern(ObjectPattern),
	ShorthandPropertyIdentifierPattern(ShorthandPropertyIdentifierPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PairKey {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PairPatternKey {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertySignatureName {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicFieldDefinitionName {
	ComputedPropertyName(ComputedPropertyName),
	Number(Number),
	PropertyIdentifier(PropertyIdentifier),
	StringLiteral(StringLiteral),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SequenceExpressionRight {
	Expression(Expression),
	SequenceExpression(SequenceExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptExpressionIndex {
	Expression(Expression),
	SequenceExpression(SequenceExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchCaseValue {
	Expression(Expression),
	SequenceExpression(SequenceExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeAliasDeclarationValue {
	PrimaryType(PrimaryType),
	ConstructorType(ConstructorType),
	FunctionType(FunctionType),
	InferType(InferType),
	IntersectionType(IntersectionType),
	ReadonlyType(ReadonlyType),
	UnionType(UnionType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryExpressionArgument {
	Expression(Expression),
	Number(Number),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryExpressionOperator {
	BangToken(BangToken),
	PlusToken(PlusToken),
	DashToken(DashToken),
	DeleteToken(DeleteToken),
	TypeofToken(TypeofToken),
	VoidToken(VoidToken),
	TildeToken(TildeToken),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateExpressionOperator {
	PlusPlusToken(PlusPlusToken),
	DashDashToken(DashDashToken),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableDeclaratorName {
	ArrayPattern(ArrayPattern),
	Identifier(Identifier),
	ObjectPattern(ObjectPattern),
}
impl AstNode for AbstractClassDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AbstractClassDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AbstractMethodSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AbstractMethodSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AccessibilityModifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AccessibilityModifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AmbientDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AmbientDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Arguments {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Arguments
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Array {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Array
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ArrayPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ArrayPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ArrayType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ArrayType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ArrowFunction {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ArrowFunction
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AsExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AsExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Asserts {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Asserts
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AssignmentExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AssignmentExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AssignmentPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AugmentedAssignmentExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AugmentedAssignmentExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AwaitExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AwaitExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BinaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BinaryExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BreakStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BreakStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CallExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CallExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CallSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CallSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CatchClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CatchClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Class {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Class
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ClassBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ClassBody
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ClassDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ClassDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ClassHeritage {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ClassHeritage
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ComputedPropertyName {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ComputedPropertyName
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ConditionalType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ConditionalType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Constraint {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Constraint
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ConstructSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ConstructSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ConstructorType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ConstructorType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ContinueStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ContinueStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DebuggerStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DebuggerStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Decorator {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Decorator
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DefaultType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DefaultType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DoStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DoStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ElseClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ElseClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EmptyStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EmptyStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EnumAssignment {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EnumAssignment
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EnumBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EnumBody
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EnumDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EnumDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExistentialType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExistentialType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExportClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExportSpecifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExportStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExportStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExpressionStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExpressionStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExtendsClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExtendsClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FinallyClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FinallyClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FlowMaybeType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FlowMaybeType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ForInStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ForInStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ForStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ForStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FormalParameters {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FormalParameters
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Function {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Function
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FunctionDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FunctionDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FunctionSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FunctionSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FunctionType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FunctionType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GeneratorFunction {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GeneratorFunction
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GeneratorFunctionDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GeneratorFunctionDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GenericType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GenericType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IfStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IfStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImplementsClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImplementsClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Import {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Import
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportAlias {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportAlias
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportRequireClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportRequireClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportSpecifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IndexSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IndexSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IndexTypeQuery {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IndexTypeQuery
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InferType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InferType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InterfaceDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InterfaceDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InternalModule {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InternalModule
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IntersectionType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IntersectionType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxAttribute {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxAttribute
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxClosingElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxClosingElement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxElement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxFragment {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxFragment
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxNamespaceName {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxNamespaceName
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxOpeningElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxOpeningElement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxSelfClosingElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxSelfClosingElement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LabeledStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LabeledStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LexicalDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LexicalDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LiteralType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LiteralType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LookupType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LookupType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for MappedTypeClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::MappedTypeClause
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for MemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::MemberExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for MetaProperty {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::MetaProperty
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for MethodDefinition {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::MethodDefinition
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for MethodSignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::MethodSignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Module {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Module
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NamedImports {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NamedImports
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NamespaceImport {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NamespaceImport
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NestedIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NestedIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NestedTypeIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NestedTypeIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NewExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NewExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NonNullExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NonNullExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Object {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Object
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ObjectAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ObjectAssignmentPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ObjectPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ObjectPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ObjectType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ObjectType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for OmittingTypeAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::OmittingTypeAnnotation
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for OptingTypeAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::OptingTypeAnnotation
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for OptionalParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::OptionalParameter
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for OptionalType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::OptionalType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Pair {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Pair
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PairPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PairPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ParenthesizedExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ParenthesizedExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ParenthesizedType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ParenthesizedType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PredefinedType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PredefinedType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Program {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Program
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PropertySignature {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PropertySignature
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PublicFieldDefinition {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PublicFieldDefinition
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ReadonlyType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ReadonlyType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Regex {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Regex
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RequiredParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RequiredParameter
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RestPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RestPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RestType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RestType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ReturnStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ReturnStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SequenceExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SequenceExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SpreadElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SpreadElement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StatementBlock {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StatementBlock
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StringLiteral {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StringLiteral
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SubscriptExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SubscriptExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SwitchBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SwitchBody
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SwitchCase {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SwitchCase
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SwitchDefault {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SwitchDefault
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SwitchStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SwitchStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TemplateString {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TemplateString
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TemplateSubstitution {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TemplateSubstitution
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TernaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TernaryExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ThrowStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ThrowStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TryStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TryStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TupleType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TupleType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeAliasDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeAliasDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeAnnotation
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeArguments {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeArguments
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeParameter
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeParameters {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeParameters
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypePredicate {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypePredicate
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypePredicateAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypePredicateAnnotation
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeQuery {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeQuery
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for UnaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::UnaryExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for UnionType {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::UnionType
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for UpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::UpdateExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for VariableDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::VariableDeclaration
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for VariableDeclarator {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::VariableDeclarator
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for WhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::WhileStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for WithStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::WithStatement
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for YieldExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::YieldExpression
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BangToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BangToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BangEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BangEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BangEqEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BangEqEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DquoteToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DquoteToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DollarLbraceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DollarLbraceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PercentToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PercentToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PercentEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PercentEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AmpToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AmpToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AmpAmpToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AmpAmpToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AmpAmpEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AmpAmpEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AmpEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AmpEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SquoteToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SquoteToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LparenToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LparenToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RparenToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RparenToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StarToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StarToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StarStarToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StarStarToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StarStarEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StarStarEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StarEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StarEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PlusToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PlusToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PlusPlusToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PlusPlusToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PlusEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PlusEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CommaToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CommaToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DashToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DashToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DashDashToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DashDashToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DashEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DashEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DashQmarkColonToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DashQmarkColonToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DotToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DotToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DotDotDotToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DotDotDotToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SlashToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SlashToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SlashEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SlashEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ColonToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ColonToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SemiToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SemiToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LtLtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LtLtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LtLtEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LtLtEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LtEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LtEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EqEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EqEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EqEqEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EqEqEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EqGtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EqGtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtGtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtGtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtGtEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtGtEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtGtGtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtGtGtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GtGtGtEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GtGtGtEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for QmarkToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::QmarkToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for QmarkDotToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::QmarkDotToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for QmarkColonToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::QmarkColonToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for QmarkQmarkToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::QmarkQmarkToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for QmarkQmarkEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::QmarkQmarkEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AtToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AtToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LbrackToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LbrackToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RbrackToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RbrackToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CaretToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CaretToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CaretEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CaretEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BquoteToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BquoteToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AbstractToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AbstractToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AnyToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AnyToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AsToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AsToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AssertsToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AssertsToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AsyncToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AsyncToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for AwaitToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::AwaitToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BooleanToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BooleanToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for BreakToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::BreakToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CaseToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CaseToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for CatchToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::CatchToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ClassToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ClassToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Comment {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Comment
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ConstToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ConstToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ContinueToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ContinueToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DebuggerToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DebuggerToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DeclareToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DeclareToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DefaultToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DefaultToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DeleteToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DeleteToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for DoToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::DoToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ElseToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ElseToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EnumToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EnumToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for EscapeSequence {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::EscapeSequence
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExportToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExportToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ExtendsToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ExtendsToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for False {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::False
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FinallyToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FinallyToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ForToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ForToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FromToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FromToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for FunctionToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::FunctionToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GetToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GetToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for GlobalToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::GlobalToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for HashBangLine {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::HashBangLine
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Identifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Identifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IfToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IfToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImplementsToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImplementsToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ImportToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ImportToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InferToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InferToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InstanceofToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InstanceofToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for InterfaceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::InterfaceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for IsToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::IsToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for JsxText {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::JsxText
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for KeyofToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::KeyofToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LetToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LetToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ModuleToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ModuleToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NamespaceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NamespaceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NewToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NewToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Null {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Null
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Number {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Number
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for NumberToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::NumberToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for OfToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::OfToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PrivateToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PrivateToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PropertyIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PropertyIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ProtectedToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ProtectedToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PublicToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PublicToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ReadonlyToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ReadonlyToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RegexFlags {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RegexFlags
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RegexPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RegexPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RequireToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RequireToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ReturnToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ReturnToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SetToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SetToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ShorthandPropertyIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ShorthandPropertyIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ShorthandPropertyIdentifierPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ShorthandPropertyIdentifierPattern
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StatementIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StatementIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StaticToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StaticToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for StringToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::StringToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Super {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Super
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SwitchToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SwitchToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for SymbolToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::SymbolToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TargetToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TargetToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for This {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::This
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for ThrowToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::ThrowToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for True {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::True
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TryToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TryToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeIdentifier
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TypeofToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TypeofToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for Undefined {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::Undefined
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for VarToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::VarToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for VoidToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::VoidToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for WhileToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::WhileToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for WithToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::WithToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for YieldToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::YieldToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LbraceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LbraceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for LbracePipeToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::LbracePipeToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PipeToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PipeToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PipeEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PipeEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PipePipeToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PipePipeToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PipePipeEqToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PipePipeEqToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for PipeRbraceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::PipeRbraceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for RbraceToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::RbraceToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl AstNode for TildeToken {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == SyntaxKind::TildeToken
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode {
		&self.syntax
	}
}
impl From<ArrayType> for PrimaryType {
	fn from(node: ArrayType) -> PrimaryType {
		PrimaryType::ArrayType(node)
	}
}
impl From<ConditionalType> for PrimaryType {
	fn from(node: ConditionalType) -> PrimaryType {
		PrimaryType::ConditionalType(node)
	}
}
impl From<ExistentialType> for PrimaryType {
	fn from(node: ExistentialType) -> PrimaryType {
		PrimaryType::ExistentialType(node)
	}
}
impl From<FlowMaybeType> for PrimaryType {
	fn from(node: FlowMaybeType) -> PrimaryType {
		PrimaryType::FlowMaybeType(node)
	}
}
impl From<GenericType> for PrimaryType {
	fn from(node: GenericType) -> PrimaryType {
		PrimaryType::GenericType(node)
	}
}
impl From<IndexTypeQuery> for PrimaryType {
	fn from(node: IndexTypeQuery) -> PrimaryType {
		PrimaryType::IndexTypeQuery(node)
	}
}
impl From<LiteralType> for PrimaryType {
	fn from(node: LiteralType) -> PrimaryType {
		PrimaryType::LiteralType(node)
	}
}
impl From<LookupType> for PrimaryType {
	fn from(node: LookupType) -> PrimaryType {
		PrimaryType::LookupType(node)
	}
}
impl From<NestedTypeIdentifier> for PrimaryType {
	fn from(node: NestedTypeIdentifier) -> PrimaryType {
		PrimaryType::NestedTypeIdentifier(node)
	}
}
impl From<ObjectType> for PrimaryType {
	fn from(node: ObjectType) -> PrimaryType {
		PrimaryType::ObjectType(node)
	}
}
impl From<ParenthesizedType> for PrimaryType {
	fn from(node: ParenthesizedType) -> PrimaryType {
		PrimaryType::ParenthesizedType(node)
	}
}
impl From<PredefinedType> for PrimaryType {
	fn from(node: PredefinedType) -> PrimaryType {
		PrimaryType::PredefinedType(node)
	}
}
impl From<This> for PrimaryType {
	fn from(node: This) -> PrimaryType {
		PrimaryType::This(node)
	}
}
impl From<TupleType> for PrimaryType {
	fn from(node: TupleType) -> PrimaryType {
		PrimaryType::TupleType(node)
	}
}
impl From<TypeIdentifier> for PrimaryType {
	fn from(node: TypeIdentifier) -> PrimaryType {
		PrimaryType::TypeIdentifier(node)
	}
}
impl From<TypeQuery> for PrimaryType {
	fn from(node: TypeQuery) -> PrimaryType {
		PrimaryType::TypeQuery(node)
	}
}
impl AstNode for PrimaryType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayType
			| SyntaxKind::ConditionalType
			| SyntaxKind::ExistentialType
			| SyntaxKind::FlowMaybeType
			| SyntaxKind::GenericType
			| SyntaxKind::IndexTypeQuery
			| SyntaxKind::LiteralType
			| SyntaxKind::LookupType
			| SyntaxKind::NestedTypeIdentifier
			| SyntaxKind::ObjectType
			| SyntaxKind::ParenthesizedType
			| SyntaxKind::PredefinedType
			| SyntaxKind::This
			| SyntaxKind::TupleType
			| SyntaxKind::TypeIdentifier
			| SyntaxKind::TypeQuery => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayType => PrimaryType::ArrayType(ArrayType { syntax }),
			SyntaxKind::ConditionalType => PrimaryType::ConditionalType(ConditionalType { syntax }),
			SyntaxKind::ExistentialType => PrimaryType::ExistentialType(ExistentialType { syntax }),
			SyntaxKind::FlowMaybeType => PrimaryType::FlowMaybeType(FlowMaybeType { syntax }),
			SyntaxKind::GenericType => PrimaryType::GenericType(GenericType { syntax }),
			SyntaxKind::IndexTypeQuery => PrimaryType::IndexTypeQuery(IndexTypeQuery { syntax }),
			SyntaxKind::LiteralType => PrimaryType::LiteralType(LiteralType { syntax }),
			SyntaxKind::LookupType => PrimaryType::LookupType(LookupType { syntax }),
			SyntaxKind::NestedTypeIdentifier => {
				PrimaryType::NestedTypeIdentifier(NestedTypeIdentifier { syntax })
			}
			SyntaxKind::ObjectType => PrimaryType::ObjectType(ObjectType { syntax }),
			SyntaxKind::ParenthesizedType => {
				PrimaryType::ParenthesizedType(ParenthesizedType { syntax })
			}
			SyntaxKind::PredefinedType => PrimaryType::PredefinedType(PredefinedType { syntax }),
			SyntaxKind::This => PrimaryType::This(This { syntax }),
			SyntaxKind::TupleType => PrimaryType::TupleType(TupleType { syntax }),
			SyntaxKind::TypeIdentifier => PrimaryType::TypeIdentifier(TypeIdentifier { syntax }),
			SyntaxKind::TypeQuery => PrimaryType::TypeQuery(TypeQuery { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PrimaryType::ArrayType(it) => &it.syntax,
			PrimaryType::ConditionalType(it) => &it.syntax,
			PrimaryType::ExistentialType(it) => &it.syntax,
			PrimaryType::FlowMaybeType(it) => &it.syntax,
			PrimaryType::GenericType(it) => &it.syntax,
			PrimaryType::IndexTypeQuery(it) => &it.syntax,
			PrimaryType::LiteralType(it) => &it.syntax,
			PrimaryType::LookupType(it) => &it.syntax,
			PrimaryType::NestedTypeIdentifier(it) => &it.syntax,
			PrimaryType::ObjectType(it) => &it.syntax,
			PrimaryType::ParenthesizedType(it) => &it.syntax,
			PrimaryType::PredefinedType(it) => &it.syntax,
			PrimaryType::This(it) => &it.syntax,
			PrimaryType::TupleType(it) => &it.syntax,
			PrimaryType::TypeIdentifier(it) => &it.syntax,
			PrimaryType::TypeQuery(it) => &it.syntax,
		}
	}
}
impl From<AbstractClassDeclaration> for Declaration {
	fn from(node: AbstractClassDeclaration) -> Declaration {
		Declaration::AbstractClassDeclaration(node)
	}
}
impl From<AmbientDeclaration> for Declaration {
	fn from(node: AmbientDeclaration) -> Declaration {
		Declaration::AmbientDeclaration(node)
	}
}
impl From<ClassDeclaration> for Declaration {
	fn from(node: ClassDeclaration) -> Declaration {
		Declaration::ClassDeclaration(node)
	}
}
impl From<EnumDeclaration> for Declaration {
	fn from(node: EnumDeclaration) -> Declaration {
		Declaration::EnumDeclaration(node)
	}
}
impl From<FunctionDeclaration> for Declaration {
	fn from(node: FunctionDeclaration) -> Declaration {
		Declaration::FunctionDeclaration(node)
	}
}
impl From<FunctionSignature> for Declaration {
	fn from(node: FunctionSignature) -> Declaration {
		Declaration::FunctionSignature(node)
	}
}
impl From<GeneratorFunctionDeclaration> for Declaration {
	fn from(node: GeneratorFunctionDeclaration) -> Declaration {
		Declaration::GeneratorFunctionDeclaration(node)
	}
}
impl From<ImportAlias> for Declaration {
	fn from(node: ImportAlias) -> Declaration {
		Declaration::ImportAlias(node)
	}
}
impl From<InterfaceDeclaration> for Declaration {
	fn from(node: InterfaceDeclaration) -> Declaration {
		Declaration::InterfaceDeclaration(node)
	}
}
impl From<InternalModule> for Declaration {
	fn from(node: InternalModule) -> Declaration {
		Declaration::InternalModule(node)
	}
}
impl From<LexicalDeclaration> for Declaration {
	fn from(node: LexicalDeclaration) -> Declaration {
		Declaration::LexicalDeclaration(node)
	}
}
impl From<Module> for Declaration {
	fn from(node: Module) -> Declaration {
		Declaration::Module(node)
	}
}
impl From<TypeAliasDeclaration> for Declaration {
	fn from(node: TypeAliasDeclaration) -> Declaration {
		Declaration::TypeAliasDeclaration(node)
	}
}
impl From<VariableDeclaration> for Declaration {
	fn from(node: VariableDeclaration) -> Declaration {
		Declaration::VariableDeclaration(node)
	}
}
impl AstNode for Declaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::AbstractClassDeclaration
			| SyntaxKind::AmbientDeclaration
			| SyntaxKind::ClassDeclaration
			| SyntaxKind::EnumDeclaration
			| SyntaxKind::FunctionDeclaration
			| SyntaxKind::FunctionSignature
			| SyntaxKind::GeneratorFunctionDeclaration
			| SyntaxKind::ImportAlias
			| SyntaxKind::InterfaceDeclaration
			| SyntaxKind::InternalModule
			| SyntaxKind::LexicalDeclaration
			| SyntaxKind::Module
			| SyntaxKind::TypeAliasDeclaration
			| SyntaxKind::VariableDeclaration => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::AbstractClassDeclaration => {
				Declaration::AbstractClassDeclaration(AbstractClassDeclaration { syntax })
			}
			SyntaxKind::AmbientDeclaration => {
				Declaration::AmbientDeclaration(AmbientDeclaration { syntax })
			}
			SyntaxKind::ClassDeclaration => {
				Declaration::ClassDeclaration(ClassDeclaration { syntax })
			}
			SyntaxKind::EnumDeclaration => Declaration::EnumDeclaration(EnumDeclaration { syntax }),
			SyntaxKind::FunctionDeclaration => {
				Declaration::FunctionDeclaration(FunctionDeclaration { syntax })
			}
			SyntaxKind::FunctionSignature => {
				Declaration::FunctionSignature(FunctionSignature { syntax })
			}
			SyntaxKind::GeneratorFunctionDeclaration => {
				Declaration::GeneratorFunctionDeclaration(GeneratorFunctionDeclaration { syntax })
			}
			SyntaxKind::ImportAlias => Declaration::ImportAlias(ImportAlias { syntax }),
			SyntaxKind::InterfaceDeclaration => {
				Declaration::InterfaceDeclaration(InterfaceDeclaration { syntax })
			}
			SyntaxKind::InternalModule => Declaration::InternalModule(InternalModule { syntax }),
			SyntaxKind::LexicalDeclaration => {
				Declaration::LexicalDeclaration(LexicalDeclaration { syntax })
			}
			SyntaxKind::Module => Declaration::Module(Module { syntax }),
			SyntaxKind::TypeAliasDeclaration => {
				Declaration::TypeAliasDeclaration(TypeAliasDeclaration { syntax })
			}
			SyntaxKind::VariableDeclaration => {
				Declaration::VariableDeclaration(VariableDeclaration { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Declaration::AbstractClassDeclaration(it) => &it.syntax,
			Declaration::AmbientDeclaration(it) => &it.syntax,
			Declaration::ClassDeclaration(it) => &it.syntax,
			Declaration::EnumDeclaration(it) => &it.syntax,
			Declaration::FunctionDeclaration(it) => &it.syntax,
			Declaration::FunctionSignature(it) => &it.syntax,
			Declaration::GeneratorFunctionDeclaration(it) => &it.syntax,
			Declaration::ImportAlias(it) => &it.syntax,
			Declaration::InterfaceDeclaration(it) => &it.syntax,
			Declaration::InternalModule(it) => &it.syntax,
			Declaration::LexicalDeclaration(it) => &it.syntax,
			Declaration::Module(it) => &it.syntax,
			Declaration::TypeAliasDeclaration(it) => &it.syntax,
			Declaration::VariableDeclaration(it) => &it.syntax,
		}
	}
}
impl From<AsExpression> for Expression {
	fn from(node: AsExpression) -> Expression {
		Expression::AsExpression(node)
	}
}
impl From<AssignmentExpression> for Expression {
	fn from(node: AssignmentExpression) -> Expression {
		Expression::AssignmentExpression(node)
	}
}
impl From<AugmentedAssignmentExpression> for Expression {
	fn from(node: AugmentedAssignmentExpression) -> Expression {
		Expression::AugmentedAssignmentExpression(node)
	}
}
impl From<AwaitExpression> for Expression {
	fn from(node: AwaitExpression) -> Expression {
		Expression::AwaitExpression(node)
	}
}
impl From<BinaryExpression> for Expression {
	fn from(node: BinaryExpression) -> Expression {
		Expression::BinaryExpression(node)
	}
}
impl From<InternalModule> for Expression {
	fn from(node: InternalModule) -> Expression {
		Expression::InternalModule(node)
	}
}
impl From<JsxElement> for Expression {
	fn from(node: JsxElement) -> Expression {
		Expression::JsxElement(node)
	}
}
impl From<JsxFragment> for Expression {
	fn from(node: JsxFragment) -> Expression {
		Expression::JsxFragment(node)
	}
}
impl From<JsxSelfClosingElement> for Expression {
	fn from(node: JsxSelfClosingElement) -> Expression {
		Expression::JsxSelfClosingElement(node)
	}
}
impl From<NewExpression> for Expression {
	fn from(node: NewExpression) -> Expression {
		Expression::NewExpression(node)
	}
}
impl From<PrimaryExpression> for Expression {
	fn from(node: PrimaryExpression) -> Expression {
		Expression::PrimaryExpression(node)
	}
}
impl From<TernaryExpression> for Expression {
	fn from(node: TernaryExpression) -> Expression {
		Expression::TernaryExpression(node)
	}
}
impl From<UnaryExpression> for Expression {
	fn from(node: UnaryExpression) -> Expression {
		Expression::UnaryExpression(node)
	}
}
impl From<UpdateExpression> for Expression {
	fn from(node: UpdateExpression) -> Expression {
		Expression::UpdateExpression(node)
	}
}
impl From<YieldExpression> for Expression {
	fn from(node: YieldExpression) -> Expression {
		Expression::YieldExpression(node)
	}
}
impl AstNode for Expression {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::AsExpression
			| SyntaxKind::AssignmentExpression
			| SyntaxKind::AugmentedAssignmentExpression
			| SyntaxKind::AwaitExpression
			| SyntaxKind::BinaryExpression
			| SyntaxKind::InternalModule
			| SyntaxKind::JsxElement
			| SyntaxKind::JsxFragment
			| SyntaxKind::JsxSelfClosingElement
			| SyntaxKind::NewExpression
			| SyntaxKind::TernaryExpression
			| SyntaxKind::UnaryExpression
			| SyntaxKind::UpdateExpression
			| SyntaxKind::YieldExpression => true,
			_ => PrimaryExpression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::AsExpression => Expression::AsExpression(AsExpression { syntax }),
			SyntaxKind::AssignmentExpression => {
				Expression::AssignmentExpression(AssignmentExpression { syntax })
			}
			SyntaxKind::AugmentedAssignmentExpression => {
				Expression::AugmentedAssignmentExpression(AugmentedAssignmentExpression { syntax })
			}
			SyntaxKind::AwaitExpression => Expression::AwaitExpression(AwaitExpression { syntax }),
			SyntaxKind::BinaryExpression => {
				Expression::BinaryExpression(BinaryExpression { syntax })
			}
			SyntaxKind::InternalModule => Expression::InternalModule(InternalModule { syntax }),
			SyntaxKind::JsxElement => Expression::JsxElement(JsxElement { syntax }),
			SyntaxKind::JsxFragment => Expression::JsxFragment(JsxFragment { syntax }),
			SyntaxKind::JsxSelfClosingElement => {
				Expression::JsxSelfClosingElement(JsxSelfClosingElement { syntax })
			}
			SyntaxKind::NewExpression => Expression::NewExpression(NewExpression { syntax }),
			SyntaxKind::TernaryExpression => {
				Expression::TernaryExpression(TernaryExpression { syntax })
			}
			SyntaxKind::UnaryExpression => Expression::UnaryExpression(UnaryExpression { syntax }),
			SyntaxKind::UpdateExpression => {
				Expression::UpdateExpression(UpdateExpression { syntax })
			}
			SyntaxKind::YieldExpression => Expression::YieldExpression(YieldExpression { syntax }),
			kind if PrimaryExpression::can_cast(kind) => {
				Expression::PrimaryExpression(PrimaryExpression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Expression::AsExpression(it) => &it.syntax,
			Expression::AssignmentExpression(it) => &it.syntax,
			Expression::AugmentedAssignmentExpression(it) => &it.syntax,
			Expression::AwaitExpression(it) => &it.syntax,
			Expression::BinaryExpression(it) => &it.syntax,
			Expression::InternalModule(it) => &it.syntax,
			Expression::JsxElement(it) => &it.syntax,
			Expression::JsxFragment(it) => &it.syntax,
			Expression::JsxSelfClosingElement(it) => &it.syntax,
			Expression::NewExpression(it) => &it.syntax,
			Expression::TernaryExpression(it) => &it.syntax,
			Expression::UnaryExpression(it) => &it.syntax,
			Expression::UpdateExpression(it) => &it.syntax,
			Expression::YieldExpression(it) => &it.syntax,
			Expression::PrimaryExpression(it) => &it.syntax(),
		}
	}
}
impl From<ArrayPattern> for Pattern {
	fn from(node: ArrayPattern) -> Pattern {
		Pattern::ArrayPattern(node)
	}
}
impl From<Identifier> for Pattern {
	fn from(node: Identifier) -> Pattern {
		Pattern::Identifier(node)
	}
}
impl From<ObjectPattern> for Pattern {
	fn from(node: ObjectPattern) -> Pattern {
		Pattern::ObjectPattern(node)
	}
}
impl From<RestPattern> for Pattern {
	fn from(node: RestPattern) -> Pattern {
		Pattern::RestPattern(node)
	}
}
impl AstNode for Pattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern
			| SyntaxKind::Identifier
			| SyntaxKind::ObjectPattern
			| SyntaxKind::RestPattern => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => Pattern::ArrayPattern(ArrayPattern { syntax }),
			SyntaxKind::Identifier => Pattern::Identifier(Identifier { syntax }),
			SyntaxKind::ObjectPattern => Pattern::ObjectPattern(ObjectPattern { syntax }),
			SyntaxKind::RestPattern => Pattern::RestPattern(RestPattern { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Pattern::ArrayPattern(it) => &it.syntax,
			Pattern::Identifier(it) => &it.syntax,
			Pattern::ObjectPattern(it) => &it.syntax,
			Pattern::RestPattern(it) => &it.syntax,
		}
	}
}
impl From<Array> for PrimaryExpression {
	fn from(node: Array) -> PrimaryExpression {
		PrimaryExpression::Array(node)
	}
}
impl From<ArrowFunction> for PrimaryExpression {
	fn from(node: ArrowFunction) -> PrimaryExpression {
		PrimaryExpression::ArrowFunction(node)
	}
}
impl From<CallExpression> for PrimaryExpression {
	fn from(node: CallExpression) -> PrimaryExpression {
		PrimaryExpression::CallExpression(node)
	}
}
impl From<Class> for PrimaryExpression {
	fn from(node: Class) -> PrimaryExpression {
		PrimaryExpression::Class(node)
	}
}
impl From<False> for PrimaryExpression {
	fn from(node: False) -> PrimaryExpression {
		PrimaryExpression::False(node)
	}
}
impl From<Function> for PrimaryExpression {
	fn from(node: Function) -> PrimaryExpression {
		PrimaryExpression::Function(node)
	}
}
impl From<GeneratorFunction> for PrimaryExpression {
	fn from(node: GeneratorFunction) -> PrimaryExpression {
		PrimaryExpression::GeneratorFunction(node)
	}
}
impl From<Identifier> for PrimaryExpression {
	fn from(node: Identifier) -> PrimaryExpression {
		PrimaryExpression::Identifier(node)
	}
}
impl From<Import> for PrimaryExpression {
	fn from(node: Import) -> PrimaryExpression {
		PrimaryExpression::Import(node)
	}
}
impl From<MemberExpression> for PrimaryExpression {
	fn from(node: MemberExpression) -> PrimaryExpression {
		PrimaryExpression::MemberExpression(node)
	}
}
impl From<MetaProperty> for PrimaryExpression {
	fn from(node: MetaProperty) -> PrimaryExpression {
		PrimaryExpression::MetaProperty(node)
	}
}
impl From<NonNullExpression> for PrimaryExpression {
	fn from(node: NonNullExpression) -> PrimaryExpression {
		PrimaryExpression::NonNullExpression(node)
	}
}
impl From<Null> for PrimaryExpression {
	fn from(node: Null) -> PrimaryExpression {
		PrimaryExpression::Null(node)
	}
}
impl From<Number> for PrimaryExpression {
	fn from(node: Number) -> PrimaryExpression {
		PrimaryExpression::Number(node)
	}
}
impl From<Object> for PrimaryExpression {
	fn from(node: Object) -> PrimaryExpression {
		PrimaryExpression::Object(node)
	}
}
impl From<ParenthesizedExpression> for PrimaryExpression {
	fn from(node: ParenthesizedExpression) -> PrimaryExpression {
		PrimaryExpression::ParenthesizedExpression(node)
	}
}
impl From<Regex> for PrimaryExpression {
	fn from(node: Regex) -> PrimaryExpression {
		PrimaryExpression::Regex(node)
	}
}
impl From<StringLiteral> for PrimaryExpression {
	fn from(node: StringLiteral) -> PrimaryExpression {
		PrimaryExpression::StringLiteral(node)
	}
}
impl From<SubscriptExpression> for PrimaryExpression {
	fn from(node: SubscriptExpression) -> PrimaryExpression {
		PrimaryExpression::SubscriptExpression(node)
	}
}
impl From<Super> for PrimaryExpression {
	fn from(node: Super) -> PrimaryExpression {
		PrimaryExpression::Super(node)
	}
}
impl From<TemplateString> for PrimaryExpression {
	fn from(node: TemplateString) -> PrimaryExpression {
		PrimaryExpression::TemplateString(node)
	}
}
impl From<This> for PrimaryExpression {
	fn from(node: This) -> PrimaryExpression {
		PrimaryExpression::This(node)
	}
}
impl From<True> for PrimaryExpression {
	fn from(node: True) -> PrimaryExpression {
		PrimaryExpression::True(node)
	}
}
impl From<Undefined> for PrimaryExpression {
	fn from(node: Undefined) -> PrimaryExpression {
		PrimaryExpression::Undefined(node)
	}
}
impl AstNode for PrimaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Array
			| SyntaxKind::ArrowFunction
			| SyntaxKind::CallExpression
			| SyntaxKind::Class
			| SyntaxKind::False
			| SyntaxKind::Function
			| SyntaxKind::GeneratorFunction
			| SyntaxKind::Identifier
			| SyntaxKind::Import
			| SyntaxKind::MemberExpression
			| SyntaxKind::MetaProperty
			| SyntaxKind::NonNullExpression
			| SyntaxKind::Null
			| SyntaxKind::Number
			| SyntaxKind::Object
			| SyntaxKind::ParenthesizedExpression
			| SyntaxKind::Regex
			| SyntaxKind::StringLiteral
			| SyntaxKind::SubscriptExpression
			| SyntaxKind::Super
			| SyntaxKind::TemplateString
			| SyntaxKind::This
			| SyntaxKind::True
			| SyntaxKind::Undefined => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Array => PrimaryExpression::Array(Array { syntax }),
			SyntaxKind::ArrowFunction => PrimaryExpression::ArrowFunction(ArrowFunction { syntax }),
			SyntaxKind::CallExpression => {
				PrimaryExpression::CallExpression(CallExpression { syntax })
			}
			SyntaxKind::Class => PrimaryExpression::Class(Class { syntax }),
			SyntaxKind::False => PrimaryExpression::False(False { syntax }),
			SyntaxKind::Function => PrimaryExpression::Function(Function { syntax }),
			SyntaxKind::GeneratorFunction => {
				PrimaryExpression::GeneratorFunction(GeneratorFunction { syntax })
			}
			SyntaxKind::Identifier => PrimaryExpression::Identifier(Identifier { syntax }),
			SyntaxKind::Import => PrimaryExpression::Import(Import { syntax }),
			SyntaxKind::MemberExpression => {
				PrimaryExpression::MemberExpression(MemberExpression { syntax })
			}
			SyntaxKind::MetaProperty => PrimaryExpression::MetaProperty(MetaProperty { syntax }),
			SyntaxKind::NonNullExpression => {
				PrimaryExpression::NonNullExpression(NonNullExpression { syntax })
			}
			SyntaxKind::Null => PrimaryExpression::Null(Null { syntax }),
			SyntaxKind::Number => PrimaryExpression::Number(Number { syntax }),
			SyntaxKind::Object => PrimaryExpression::Object(Object { syntax }),
			SyntaxKind::ParenthesizedExpression => {
				PrimaryExpression::ParenthesizedExpression(ParenthesizedExpression { syntax })
			}
			SyntaxKind::Regex => PrimaryExpression::Regex(Regex { syntax }),
			SyntaxKind::StringLiteral => PrimaryExpression::StringLiteral(StringLiteral { syntax }),
			SyntaxKind::SubscriptExpression => {
				PrimaryExpression::SubscriptExpression(SubscriptExpression { syntax })
			}
			SyntaxKind::Super => PrimaryExpression::Super(Super { syntax }),
			SyntaxKind::TemplateString => {
				PrimaryExpression::TemplateString(TemplateString { syntax })
			}
			SyntaxKind::This => PrimaryExpression::This(This { syntax }),
			SyntaxKind::True => PrimaryExpression::True(True { syntax }),
			SyntaxKind::Undefined => PrimaryExpression::Undefined(Undefined { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PrimaryExpression::Array(it) => &it.syntax,
			PrimaryExpression::ArrowFunction(it) => &it.syntax,
			PrimaryExpression::CallExpression(it) => &it.syntax,
			PrimaryExpression::Class(it) => &it.syntax,
			PrimaryExpression::False(it) => &it.syntax,
			PrimaryExpression::Function(it) => &it.syntax,
			PrimaryExpression::GeneratorFunction(it) => &it.syntax,
			PrimaryExpression::Identifier(it) => &it.syntax,
			PrimaryExpression::Import(it) => &it.syntax,
			PrimaryExpression::MemberExpression(it) => &it.syntax,
			PrimaryExpression::MetaProperty(it) => &it.syntax,
			PrimaryExpression::NonNullExpression(it) => &it.syntax,
			PrimaryExpression::Null(it) => &it.syntax,
			PrimaryExpression::Number(it) => &it.syntax,
			PrimaryExpression::Object(it) => &it.syntax,
			PrimaryExpression::ParenthesizedExpression(it) => &it.syntax,
			PrimaryExpression::Regex(it) => &it.syntax,
			PrimaryExpression::StringLiteral(it) => &it.syntax,
			PrimaryExpression::SubscriptExpression(it) => &it.syntax,
			PrimaryExpression::Super(it) => &it.syntax,
			PrimaryExpression::TemplateString(it) => &it.syntax,
			PrimaryExpression::This(it) => &it.syntax,
			PrimaryExpression::True(it) => &it.syntax,
			PrimaryExpression::Undefined(it) => &it.syntax,
		}
	}
}
impl From<BreakStatement> for Statement {
	fn from(node: BreakStatement) -> Statement {
		Statement::BreakStatement(node)
	}
}
impl From<ContinueStatement> for Statement {
	fn from(node: ContinueStatement) -> Statement {
		Statement::ContinueStatement(node)
	}
}
impl From<DebuggerStatement> for Statement {
	fn from(node: DebuggerStatement) -> Statement {
		Statement::DebuggerStatement(node)
	}
}
impl From<Declaration> for Statement {
	fn from(node: Declaration) -> Statement {
		Statement::Declaration(node)
	}
}
impl From<DoStatement> for Statement {
	fn from(node: DoStatement) -> Statement {
		Statement::DoStatement(node)
	}
}
impl From<EmptyStatement> for Statement {
	fn from(node: EmptyStatement) -> Statement {
		Statement::EmptyStatement(node)
	}
}
impl From<ExportStatement> for Statement {
	fn from(node: ExportStatement) -> Statement {
		Statement::ExportStatement(node)
	}
}
impl From<ExpressionStatement> for Statement {
	fn from(node: ExpressionStatement) -> Statement {
		Statement::ExpressionStatement(node)
	}
}
impl From<ForInStatement> for Statement {
	fn from(node: ForInStatement) -> Statement {
		Statement::ForInStatement(node)
	}
}
impl From<ForStatement> for Statement {
	fn from(node: ForStatement) -> Statement {
		Statement::ForStatement(node)
	}
}
impl From<IfStatement> for Statement {
	fn from(node: IfStatement) -> Statement {
		Statement::IfStatement(node)
	}
}
impl From<ImportStatement> for Statement {
	fn from(node: ImportStatement) -> Statement {
		Statement::ImportStatement(node)
	}
}
impl From<LabeledStatement> for Statement {
	fn from(node: LabeledStatement) -> Statement {
		Statement::LabeledStatement(node)
	}
}
impl From<ReturnStatement> for Statement {
	fn from(node: ReturnStatement) -> Statement {
		Statement::ReturnStatement(node)
	}
}
impl From<StatementBlock> for Statement {
	fn from(node: StatementBlock) -> Statement {
		Statement::StatementBlock(node)
	}
}
impl From<SwitchStatement> for Statement {
	fn from(node: SwitchStatement) -> Statement {
		Statement::SwitchStatement(node)
	}
}
impl From<ThrowStatement> for Statement {
	fn from(node: ThrowStatement) -> Statement {
		Statement::ThrowStatement(node)
	}
}
impl From<TryStatement> for Statement {
	fn from(node: TryStatement) -> Statement {
		Statement::TryStatement(node)
	}
}
impl From<WhileStatement> for Statement {
	fn from(node: WhileStatement) -> Statement {
		Statement::WhileStatement(node)
	}
}
impl From<WithStatement> for Statement {
	fn from(node: WithStatement) -> Statement {
		Statement::WithStatement(node)
	}
}
impl AstNode for Statement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::BreakStatement
			| SyntaxKind::ContinueStatement
			| SyntaxKind::DebuggerStatement
			| SyntaxKind::DoStatement
			| SyntaxKind::EmptyStatement
			| SyntaxKind::ExportStatement
			| SyntaxKind::ExpressionStatement
			| SyntaxKind::ForInStatement
			| SyntaxKind::ForStatement
			| SyntaxKind::IfStatement
			| SyntaxKind::ImportStatement
			| SyntaxKind::LabeledStatement
			| SyntaxKind::ReturnStatement
			| SyntaxKind::StatementBlock
			| SyntaxKind::SwitchStatement
			| SyntaxKind::ThrowStatement
			| SyntaxKind::TryStatement
			| SyntaxKind::WhileStatement
			| SyntaxKind::WithStatement => true,
			_ => Declaration::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::BreakStatement => Statement::BreakStatement(BreakStatement { syntax }),
			SyntaxKind::ContinueStatement => {
				Statement::ContinueStatement(ContinueStatement { syntax })
			}
			SyntaxKind::DebuggerStatement => {
				Statement::DebuggerStatement(DebuggerStatement { syntax })
			}
			SyntaxKind::DoStatement => Statement::DoStatement(DoStatement { syntax }),
			SyntaxKind::EmptyStatement => Statement::EmptyStatement(EmptyStatement { syntax }),
			SyntaxKind::ExportStatement => Statement::ExportStatement(ExportStatement { syntax }),
			SyntaxKind::ExpressionStatement => {
				Statement::ExpressionStatement(ExpressionStatement { syntax })
			}
			SyntaxKind::ForInStatement => Statement::ForInStatement(ForInStatement { syntax }),
			SyntaxKind::ForStatement => Statement::ForStatement(ForStatement { syntax }),
			SyntaxKind::IfStatement => Statement::IfStatement(IfStatement { syntax }),
			SyntaxKind::ImportStatement => Statement::ImportStatement(ImportStatement { syntax }),
			SyntaxKind::LabeledStatement => {
				Statement::LabeledStatement(LabeledStatement { syntax })
			}
			SyntaxKind::ReturnStatement => Statement::ReturnStatement(ReturnStatement { syntax }),
			SyntaxKind::StatementBlock => Statement::StatementBlock(StatementBlock { syntax }),
			SyntaxKind::SwitchStatement => Statement::SwitchStatement(SwitchStatement { syntax }),
			SyntaxKind::ThrowStatement => Statement::ThrowStatement(ThrowStatement { syntax }),
			SyntaxKind::TryStatement => Statement::TryStatement(TryStatement { syntax }),
			SyntaxKind::WhileStatement => Statement::WhileStatement(WhileStatement { syntax }),
			SyntaxKind::WithStatement => Statement::WithStatement(WithStatement { syntax }),
			kind if Declaration::can_cast(kind) => {
				Statement::Declaration(Declaration::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Statement::BreakStatement(it) => &it.syntax,
			Statement::ContinueStatement(it) => &it.syntax,
			Statement::DebuggerStatement(it) => &it.syntax,
			Statement::DoStatement(it) => &it.syntax,
			Statement::EmptyStatement(it) => &it.syntax,
			Statement::ExportStatement(it) => &it.syntax,
			Statement::ExpressionStatement(it) => &it.syntax,
			Statement::ForInStatement(it) => &it.syntax,
			Statement::ForStatement(it) => &it.syntax,
			Statement::IfStatement(it) => &it.syntax,
			Statement::ImportStatement(it) => &it.syntax,
			Statement::LabeledStatement(it) => &it.syntax,
			Statement::ReturnStatement(it) => &it.syntax,
			Statement::StatementBlock(it) => &it.syntax,
			Statement::SwitchStatement(it) => &it.syntax,
			Statement::ThrowStatement(it) => &it.syntax,
			Statement::TryStatement(it) => &it.syntax,
			Statement::WhileStatement(it) => &it.syntax,
			Statement::WithStatement(it) => &it.syntax,
			Statement::Declaration(it) => &it.syntax(),
		}
	}
}
impl From<ComputedPropertyName> for AbstractMethodSignatureName {
	fn from(node: ComputedPropertyName) -> AbstractMethodSignatureName {
		AbstractMethodSignatureName::ComputedPropertyName(node)
	}
}
impl From<Number> for AbstractMethodSignatureName {
	fn from(node: Number) -> AbstractMethodSignatureName {
		AbstractMethodSignatureName::Number(node)
	}
}
impl From<PropertyIdentifier> for AbstractMethodSignatureName {
	fn from(node: PropertyIdentifier) -> AbstractMethodSignatureName {
		AbstractMethodSignatureName::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for AbstractMethodSignatureName {
	fn from(node: StringLiteral) -> AbstractMethodSignatureName {
		AbstractMethodSignatureName::StringLiteral(node)
	}
}
impl AstNode for AbstractMethodSignatureName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				AbstractMethodSignatureName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => AbstractMethodSignatureName::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				AbstractMethodSignatureName::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				AbstractMethodSignatureName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AbstractMethodSignatureName::ComputedPropertyName(it) => &it.syntax,
			AbstractMethodSignatureName::Number(it) => &it.syntax,
			AbstractMethodSignatureName::PropertyIdentifier(it) => &it.syntax,
			AbstractMethodSignatureName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for AbstractMethodSignatureReturnType {
	fn from(node: Asserts) -> AbstractMethodSignatureReturnType {
		AbstractMethodSignatureReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for AbstractMethodSignatureReturnType {
	fn from(node: TypeAnnotation) -> AbstractMethodSignatureReturnType {
		AbstractMethodSignatureReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for AbstractMethodSignatureReturnType {
	fn from(node: TypePredicateAnnotation) -> AbstractMethodSignatureReturnType {
		AbstractMethodSignatureReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for AbstractMethodSignatureReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => AbstractMethodSignatureReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				AbstractMethodSignatureReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				AbstractMethodSignatureReturnType::TypePredicateAnnotation(
					TypePredicateAnnotation { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AbstractMethodSignatureReturnType::Asserts(it) => &it.syntax,
			AbstractMethodSignatureReturnType::TypeAnnotation(it) => &it.syntax,
			AbstractMethodSignatureReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Expression> for ArrowFunctionBody {
	fn from(node: Expression) -> ArrowFunctionBody {
		ArrowFunctionBody::Expression(node)
	}
}
impl From<StatementBlock> for ArrowFunctionBody {
	fn from(node: StatementBlock) -> ArrowFunctionBody {
		ArrowFunctionBody::StatementBlock(node)
	}
}
impl AstNode for ArrowFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::StatementBlock => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::StatementBlock => {
				ArrowFunctionBody::StatementBlock(StatementBlock { syntax })
			}
			kind if Expression::can_cast(kind) => {
				ArrowFunctionBody::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ArrowFunctionBody::StatementBlock(it) => &it.syntax,
			ArrowFunctionBody::Expression(it) => &it.syntax(),
		}
	}
}
impl From<Asserts> for ArrowFunctionReturnType {
	fn from(node: Asserts) -> ArrowFunctionReturnType {
		ArrowFunctionReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for ArrowFunctionReturnType {
	fn from(node: TypeAnnotation) -> ArrowFunctionReturnType {
		ArrowFunctionReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for ArrowFunctionReturnType {
	fn from(node: TypePredicateAnnotation) -> ArrowFunctionReturnType {
		ArrowFunctionReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for ArrowFunctionReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => ArrowFunctionReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				ArrowFunctionReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				ArrowFunctionReturnType::TypePredicateAnnotation(TypePredicateAnnotation { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ArrowFunctionReturnType::Asserts(it) => &it.syntax,
			ArrowFunctionReturnType::TypeAnnotation(it) => &it.syntax,
			ArrowFunctionReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<ArrayPattern> for AssignmentExpressionLeft {
	fn from(node: ArrayPattern) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::ArrayPattern(node)
	}
}
impl From<Identifier> for AssignmentExpressionLeft {
	fn from(node: Identifier) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::Identifier(node)
	}
}
impl From<MemberExpression> for AssignmentExpressionLeft {
	fn from(node: MemberExpression) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::MemberExpression(node)
	}
}
impl From<NonNullExpression> for AssignmentExpressionLeft {
	fn from(node: NonNullExpression) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::NonNullExpression(node)
	}
}
impl From<ObjectPattern> for AssignmentExpressionLeft {
	fn from(node: ObjectPattern) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::ObjectPattern(node)
	}
}
impl From<ParenthesizedExpression> for AssignmentExpressionLeft {
	fn from(node: ParenthesizedExpression) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::ParenthesizedExpression(node)
	}
}
impl From<SubscriptExpression> for AssignmentExpressionLeft {
	fn from(node: SubscriptExpression) -> AssignmentExpressionLeft {
		AssignmentExpressionLeft::SubscriptExpression(node)
	}
}
impl AstNode for AssignmentExpressionLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern
			| SyntaxKind::Identifier
			| SyntaxKind::MemberExpression
			| SyntaxKind::NonNullExpression
			| SyntaxKind::ObjectPattern
			| SyntaxKind::ParenthesizedExpression
			| SyntaxKind::SubscriptExpression => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => {
				AssignmentExpressionLeft::ArrayPattern(ArrayPattern { syntax })
			}
			SyntaxKind::Identifier => AssignmentExpressionLeft::Identifier(Identifier { syntax }),
			SyntaxKind::MemberExpression => {
				AssignmentExpressionLeft::MemberExpression(MemberExpression { syntax })
			}
			SyntaxKind::NonNullExpression => {
				AssignmentExpressionLeft::NonNullExpression(NonNullExpression { syntax })
			}
			SyntaxKind::ObjectPattern => {
				AssignmentExpressionLeft::ObjectPattern(ObjectPattern { syntax })
			}
			SyntaxKind::ParenthesizedExpression => {
				AssignmentExpressionLeft::ParenthesizedExpression(ParenthesizedExpression {
					syntax,
				})
			}
			SyntaxKind::SubscriptExpression => {
				AssignmentExpressionLeft::SubscriptExpression(SubscriptExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AssignmentExpressionLeft::ArrayPattern(it) => &it.syntax,
			AssignmentExpressionLeft::Identifier(it) => &it.syntax,
			AssignmentExpressionLeft::MemberExpression(it) => &it.syntax,
			AssignmentExpressionLeft::NonNullExpression(it) => &it.syntax,
			AssignmentExpressionLeft::ObjectPattern(it) => &it.syntax,
			AssignmentExpressionLeft::ParenthesizedExpression(it) => &it.syntax,
			AssignmentExpressionLeft::SubscriptExpression(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for AugmentedAssignmentExpressionLeft {
	fn from(node: Identifier) -> AugmentedAssignmentExpressionLeft {
		AugmentedAssignmentExpressionLeft::Identifier(node)
	}
}
impl From<MemberExpression> for AugmentedAssignmentExpressionLeft {
	fn from(node: MemberExpression) -> AugmentedAssignmentExpressionLeft {
		AugmentedAssignmentExpressionLeft::MemberExpression(node)
	}
}
impl From<NonNullExpression> for AugmentedAssignmentExpressionLeft {
	fn from(node: NonNullExpression) -> AugmentedAssignmentExpressionLeft {
		AugmentedAssignmentExpressionLeft::NonNullExpression(node)
	}
}
impl From<ParenthesizedExpression> for AugmentedAssignmentExpressionLeft {
	fn from(node: ParenthesizedExpression) -> AugmentedAssignmentExpressionLeft {
		AugmentedAssignmentExpressionLeft::ParenthesizedExpression(node)
	}
}
impl From<SubscriptExpression> for AugmentedAssignmentExpressionLeft {
	fn from(node: SubscriptExpression) -> AugmentedAssignmentExpressionLeft {
		AugmentedAssignmentExpressionLeft::SubscriptExpression(node)
	}
}
impl AstNode for AugmentedAssignmentExpressionLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier
			| SyntaxKind::MemberExpression
			| SyntaxKind::NonNullExpression
			| SyntaxKind::ParenthesizedExpression
			| SyntaxKind::SubscriptExpression => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => {
				AugmentedAssignmentExpressionLeft::Identifier(Identifier { syntax })
			}
			SyntaxKind::MemberExpression => {
				AugmentedAssignmentExpressionLeft::MemberExpression(MemberExpression { syntax })
			}
			SyntaxKind::NonNullExpression => {
				AugmentedAssignmentExpressionLeft::NonNullExpression(NonNullExpression { syntax })
			}
			SyntaxKind::ParenthesizedExpression => {
				AugmentedAssignmentExpressionLeft::ParenthesizedExpression(
					ParenthesizedExpression { syntax },
				)
			}
			SyntaxKind::SubscriptExpression => {
				AugmentedAssignmentExpressionLeft::SubscriptExpression(SubscriptExpression {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AugmentedAssignmentExpressionLeft::Identifier(it) => &it.syntax,
			AugmentedAssignmentExpressionLeft::MemberExpression(it) => &it.syntax,
			AugmentedAssignmentExpressionLeft::NonNullExpression(it) => &it.syntax,
			AugmentedAssignmentExpressionLeft::ParenthesizedExpression(it) => &it.syntax,
			AugmentedAssignmentExpressionLeft::SubscriptExpression(it) => &it.syntax,
		}
	}
}
impl From<BangEqToken> for BinaryExpressionOperator {
	fn from(node: BangEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::BangEqToken(node)
	}
}
impl From<BangEqEqToken> for BinaryExpressionOperator {
	fn from(node: BangEqEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::BangEqEqToken(node)
	}
}
impl From<PercentToken> for BinaryExpressionOperator {
	fn from(node: PercentToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::PercentToken(node)
	}
}
impl From<AmpToken> for BinaryExpressionOperator {
	fn from(node: AmpToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::AmpToken(node)
	}
}
impl From<AmpAmpToken> for BinaryExpressionOperator {
	fn from(node: AmpAmpToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::AmpAmpToken(node)
	}
}
impl From<StarToken> for BinaryExpressionOperator {
	fn from(node: StarToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::StarToken(node)
	}
}
impl From<StarStarToken> for BinaryExpressionOperator {
	fn from(node: StarStarToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::StarStarToken(node)
	}
}
impl From<PlusToken> for BinaryExpressionOperator {
	fn from(node: PlusToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::PlusToken(node)
	}
}
impl From<DashToken> for BinaryExpressionOperator {
	fn from(node: DashToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::DashToken(node)
	}
}
impl From<SlashToken> for BinaryExpressionOperator {
	fn from(node: SlashToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::SlashToken(node)
	}
}
impl From<LtToken> for BinaryExpressionOperator {
	fn from(node: LtToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::LtToken(node)
	}
}
impl From<LtLtToken> for BinaryExpressionOperator {
	fn from(node: LtLtToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::LtLtToken(node)
	}
}
impl From<LtEqToken> for BinaryExpressionOperator {
	fn from(node: LtEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::LtEqToken(node)
	}
}
impl From<EqEqToken> for BinaryExpressionOperator {
	fn from(node: EqEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::EqEqToken(node)
	}
}
impl From<EqEqEqToken> for BinaryExpressionOperator {
	fn from(node: EqEqEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::EqEqEqToken(node)
	}
}
impl From<GtToken> for BinaryExpressionOperator {
	fn from(node: GtToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::GtToken(node)
	}
}
impl From<GtEqToken> for BinaryExpressionOperator {
	fn from(node: GtEqToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::GtEqToken(node)
	}
}
impl From<GtGtToken> for BinaryExpressionOperator {
	fn from(node: GtGtToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::GtGtToken(node)
	}
}
impl From<GtGtGtToken> for BinaryExpressionOperator {
	fn from(node: GtGtGtToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::GtGtGtToken(node)
	}
}
impl From<QmarkQmarkToken> for BinaryExpressionOperator {
	fn from(node: QmarkQmarkToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::QmarkQmarkToken(node)
	}
}
impl From<CaretToken> for BinaryExpressionOperator {
	fn from(node: CaretToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::CaretToken(node)
	}
}
impl From<InToken> for BinaryExpressionOperator {
	fn from(node: InToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::InToken(node)
	}
}
impl From<InstanceofToken> for BinaryExpressionOperator {
	fn from(node: InstanceofToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::InstanceofToken(node)
	}
}
impl From<PipeToken> for BinaryExpressionOperator {
	fn from(node: PipeToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::PipeToken(node)
	}
}
impl From<PipePipeToken> for BinaryExpressionOperator {
	fn from(node: PipePipeToken) -> BinaryExpressionOperator {
		BinaryExpressionOperator::PipePipeToken(node)
	}
}
impl AstNode for BinaryExpressionOperator {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::BangEqToken
			| SyntaxKind::BangEqEqToken
			| SyntaxKind::PercentToken
			| SyntaxKind::AmpToken
			| SyntaxKind::AmpAmpToken
			| SyntaxKind::StarToken
			| SyntaxKind::StarStarToken
			| SyntaxKind::PlusToken
			| SyntaxKind::DashToken
			| SyntaxKind::SlashToken
			| SyntaxKind::LtToken
			| SyntaxKind::LtLtToken
			| SyntaxKind::LtEqToken
			| SyntaxKind::EqEqToken
			| SyntaxKind::EqEqEqToken
			| SyntaxKind::GtToken
			| SyntaxKind::GtEqToken
			| SyntaxKind::GtGtToken
			| SyntaxKind::GtGtGtToken
			| SyntaxKind::QmarkQmarkToken
			| SyntaxKind::CaretToken
			| SyntaxKind::InToken
			| SyntaxKind::InstanceofToken
			| SyntaxKind::PipeToken
			| SyntaxKind::PipePipeToken => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::BangEqToken => {
				BinaryExpressionOperator::BangEqToken(BangEqToken { syntax })
			}
			SyntaxKind::BangEqEqToken => {
				BinaryExpressionOperator::BangEqEqToken(BangEqEqToken { syntax })
			}
			SyntaxKind::PercentToken => {
				BinaryExpressionOperator::PercentToken(PercentToken { syntax })
			}
			SyntaxKind::AmpToken => BinaryExpressionOperator::AmpToken(AmpToken { syntax }),
			SyntaxKind::AmpAmpToken => {
				BinaryExpressionOperator::AmpAmpToken(AmpAmpToken { syntax })
			}
			SyntaxKind::StarToken => BinaryExpressionOperator::StarToken(StarToken { syntax }),
			SyntaxKind::StarStarToken => {
				BinaryExpressionOperator::StarStarToken(StarStarToken { syntax })
			}
			SyntaxKind::PlusToken => BinaryExpressionOperator::PlusToken(PlusToken { syntax }),
			SyntaxKind::DashToken => BinaryExpressionOperator::DashToken(DashToken { syntax }),
			SyntaxKind::SlashToken => BinaryExpressionOperator::SlashToken(SlashToken { syntax }),
			SyntaxKind::LtToken => BinaryExpressionOperator::LtToken(LtToken { syntax }),
			SyntaxKind::LtLtToken => BinaryExpressionOperator::LtLtToken(LtLtToken { syntax }),
			SyntaxKind::LtEqToken => BinaryExpressionOperator::LtEqToken(LtEqToken { syntax }),
			SyntaxKind::EqEqToken => BinaryExpressionOperator::EqEqToken(EqEqToken { syntax }),
			SyntaxKind::EqEqEqToken => {
				BinaryExpressionOperator::EqEqEqToken(EqEqEqToken { syntax })
			}
			SyntaxKind::GtToken => BinaryExpressionOperator::GtToken(GtToken { syntax }),
			SyntaxKind::GtEqToken => BinaryExpressionOperator::GtEqToken(GtEqToken { syntax }),
			SyntaxKind::GtGtToken => BinaryExpressionOperator::GtGtToken(GtGtToken { syntax }),
			SyntaxKind::GtGtGtToken => {
				BinaryExpressionOperator::GtGtGtToken(GtGtGtToken { syntax })
			}
			SyntaxKind::QmarkQmarkToken => {
				BinaryExpressionOperator::QmarkQmarkToken(QmarkQmarkToken { syntax })
			}
			SyntaxKind::CaretToken => BinaryExpressionOperator::CaretToken(CaretToken { syntax }),
			SyntaxKind::InToken => BinaryExpressionOperator::InToken(InToken { syntax }),
			SyntaxKind::InstanceofToken => {
				BinaryExpressionOperator::InstanceofToken(InstanceofToken { syntax })
			}
			SyntaxKind::PipeToken => BinaryExpressionOperator::PipeToken(PipeToken { syntax }),
			SyntaxKind::PipePipeToken => {
				BinaryExpressionOperator::PipePipeToken(PipePipeToken { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			BinaryExpressionOperator::BangEqToken(it) => &it.syntax,
			BinaryExpressionOperator::BangEqEqToken(it) => &it.syntax,
			BinaryExpressionOperator::PercentToken(it) => &it.syntax,
			BinaryExpressionOperator::AmpToken(it) => &it.syntax,
			BinaryExpressionOperator::AmpAmpToken(it) => &it.syntax,
			BinaryExpressionOperator::StarToken(it) => &it.syntax,
			BinaryExpressionOperator::StarStarToken(it) => &it.syntax,
			BinaryExpressionOperator::PlusToken(it) => &it.syntax,
			BinaryExpressionOperator::DashToken(it) => &it.syntax,
			BinaryExpressionOperator::SlashToken(it) => &it.syntax,
			BinaryExpressionOperator::LtToken(it) => &it.syntax,
			BinaryExpressionOperator::LtLtToken(it) => &it.syntax,
			BinaryExpressionOperator::LtEqToken(it) => &it.syntax,
			BinaryExpressionOperator::EqEqToken(it) => &it.syntax,
			BinaryExpressionOperator::EqEqEqToken(it) => &it.syntax,
			BinaryExpressionOperator::GtToken(it) => &it.syntax,
			BinaryExpressionOperator::GtEqToken(it) => &it.syntax,
			BinaryExpressionOperator::GtGtToken(it) => &it.syntax,
			BinaryExpressionOperator::GtGtGtToken(it) => &it.syntax,
			BinaryExpressionOperator::QmarkQmarkToken(it) => &it.syntax,
			BinaryExpressionOperator::CaretToken(it) => &it.syntax,
			BinaryExpressionOperator::InToken(it) => &it.syntax,
			BinaryExpressionOperator::InstanceofToken(it) => &it.syntax,
			BinaryExpressionOperator::PipeToken(it) => &it.syntax,
			BinaryExpressionOperator::PipePipeToken(it) => &it.syntax,
		}
	}
}
impl From<Arguments> for CallExpressionArguments {
	fn from(node: Arguments) -> CallExpressionArguments {
		CallExpressionArguments::Arguments(node)
	}
}
impl From<TemplateString> for CallExpressionArguments {
	fn from(node: TemplateString) -> CallExpressionArguments {
		CallExpressionArguments::TemplateString(node)
	}
}
impl AstNode for CallExpressionArguments {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Arguments | SyntaxKind::TemplateString => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Arguments => CallExpressionArguments::Arguments(Arguments { syntax }),
			SyntaxKind::TemplateString => {
				CallExpressionArguments::TemplateString(TemplateString { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			CallExpressionArguments::Arguments(it) => &it.syntax,
			CallExpressionArguments::TemplateString(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for CallSignatureReturnType {
	fn from(node: Asserts) -> CallSignatureReturnType {
		CallSignatureReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for CallSignatureReturnType {
	fn from(node: TypeAnnotation) -> CallSignatureReturnType {
		CallSignatureReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for CallSignatureReturnType {
	fn from(node: TypePredicateAnnotation) -> CallSignatureReturnType {
		CallSignatureReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for CallSignatureReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => CallSignatureReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				CallSignatureReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				CallSignatureReturnType::TypePredicateAnnotation(TypePredicateAnnotation { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			CallSignatureReturnType::Asserts(it) => &it.syntax,
			CallSignatureReturnType::TypeAnnotation(it) => &it.syntax,
			CallSignatureReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<ArrayPattern> for CatchClauseParameter {
	fn from(node: ArrayPattern) -> CatchClauseParameter {
		CatchClauseParameter::ArrayPattern(node)
	}
}
impl From<Identifier> for CatchClauseParameter {
	fn from(node: Identifier) -> CatchClauseParameter {
		CatchClauseParameter::Identifier(node)
	}
}
impl From<ObjectPattern> for CatchClauseParameter {
	fn from(node: ObjectPattern) -> CatchClauseParameter {
		CatchClauseParameter::ObjectPattern(node)
	}
}
impl AstNode for CatchClauseParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern | SyntaxKind::Identifier | SyntaxKind::ObjectPattern => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => CatchClauseParameter::ArrayPattern(ArrayPattern { syntax }),
			SyntaxKind::Identifier => CatchClauseParameter::Identifier(Identifier { syntax }),
			SyntaxKind::ObjectPattern => {
				CatchClauseParameter::ObjectPattern(ObjectPattern { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			CatchClauseParameter::ArrayPattern(it) => &it.syntax,
			CatchClauseParameter::Identifier(it) => &it.syntax,
			CatchClauseParameter::ObjectPattern(it) => &it.syntax,
		}
	}
}
impl From<PrimaryType> for ConditionalTypeAlternative {
	fn from(node: PrimaryType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::PrimaryType(node)
	}
}
impl From<ConstructorType> for ConditionalTypeAlternative {
	fn from(node: ConstructorType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::ConstructorType(node)
	}
}
impl From<FunctionType> for ConditionalTypeAlternative {
	fn from(node: FunctionType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::FunctionType(node)
	}
}
impl From<InferType> for ConditionalTypeAlternative {
	fn from(node: InferType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::InferType(node)
	}
}
impl From<IntersectionType> for ConditionalTypeAlternative {
	fn from(node: IntersectionType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::IntersectionType(node)
	}
}
impl From<ReadonlyType> for ConditionalTypeAlternative {
	fn from(node: ReadonlyType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::ReadonlyType(node)
	}
}
impl From<UnionType> for ConditionalTypeAlternative {
	fn from(node: UnionType) -> ConditionalTypeAlternative {
		ConditionalTypeAlternative::UnionType(node)
	}
}
impl AstNode for ConditionalTypeAlternative {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ConstructorType
			| SyntaxKind::FunctionType
			| SyntaxKind::InferType
			| SyntaxKind::IntersectionType
			| SyntaxKind::ReadonlyType
			| SyntaxKind::UnionType => true,
			_ => PrimaryType::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ConstructorType => {
				ConditionalTypeAlternative::ConstructorType(ConstructorType { syntax })
			}
			SyntaxKind::FunctionType => {
				ConditionalTypeAlternative::FunctionType(FunctionType { syntax })
			}
			SyntaxKind::InferType => ConditionalTypeAlternative::InferType(InferType { syntax }),
			SyntaxKind::IntersectionType => {
				ConditionalTypeAlternative::IntersectionType(IntersectionType { syntax })
			}
			SyntaxKind::ReadonlyType => {
				ConditionalTypeAlternative::ReadonlyType(ReadonlyType { syntax })
			}
			SyntaxKind::UnionType => ConditionalTypeAlternative::UnionType(UnionType { syntax }),
			kind if PrimaryType::can_cast(kind) => {
				ConditionalTypeAlternative::PrimaryType(PrimaryType::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConditionalTypeAlternative::ConstructorType(it) => &it.syntax,
			ConditionalTypeAlternative::FunctionType(it) => &it.syntax,
			ConditionalTypeAlternative::InferType(it) => &it.syntax,
			ConditionalTypeAlternative::IntersectionType(it) => &it.syntax,
			ConditionalTypeAlternative::ReadonlyType(it) => &it.syntax,
			ConditionalTypeAlternative::UnionType(it) => &it.syntax,
			ConditionalTypeAlternative::PrimaryType(it) => &it.syntax(),
		}
	}
}
impl From<PrimaryType> for ConditionalTypeConsequence {
	fn from(node: PrimaryType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::PrimaryType(node)
	}
}
impl From<ConstructorType> for ConditionalTypeConsequence {
	fn from(node: ConstructorType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::ConstructorType(node)
	}
}
impl From<FunctionType> for ConditionalTypeConsequence {
	fn from(node: FunctionType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::FunctionType(node)
	}
}
impl From<InferType> for ConditionalTypeConsequence {
	fn from(node: InferType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::InferType(node)
	}
}
impl From<IntersectionType> for ConditionalTypeConsequence {
	fn from(node: IntersectionType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::IntersectionType(node)
	}
}
impl From<ReadonlyType> for ConditionalTypeConsequence {
	fn from(node: ReadonlyType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::ReadonlyType(node)
	}
}
impl From<UnionType> for ConditionalTypeConsequence {
	fn from(node: UnionType) -> ConditionalTypeConsequence {
		ConditionalTypeConsequence::UnionType(node)
	}
}
impl AstNode for ConditionalTypeConsequence {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ConstructorType
			| SyntaxKind::FunctionType
			| SyntaxKind::InferType
			| SyntaxKind::IntersectionType
			| SyntaxKind::ReadonlyType
			| SyntaxKind::UnionType => true,
			_ => PrimaryType::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ConstructorType => {
				ConditionalTypeConsequence::ConstructorType(ConstructorType { syntax })
			}
			SyntaxKind::FunctionType => {
				ConditionalTypeConsequence::FunctionType(FunctionType { syntax })
			}
			SyntaxKind::InferType => ConditionalTypeConsequence::InferType(InferType { syntax }),
			SyntaxKind::IntersectionType => {
				ConditionalTypeConsequence::IntersectionType(IntersectionType { syntax })
			}
			SyntaxKind::ReadonlyType => {
				ConditionalTypeConsequence::ReadonlyType(ReadonlyType { syntax })
			}
			SyntaxKind::UnionType => ConditionalTypeConsequence::UnionType(UnionType { syntax }),
			kind if PrimaryType::can_cast(kind) => {
				ConditionalTypeConsequence::PrimaryType(PrimaryType::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConditionalTypeConsequence::ConstructorType(it) => &it.syntax,
			ConditionalTypeConsequence::FunctionType(it) => &it.syntax,
			ConditionalTypeConsequence::InferType(it) => &it.syntax,
			ConditionalTypeConsequence::IntersectionType(it) => &it.syntax,
			ConditionalTypeConsequence::ReadonlyType(it) => &it.syntax,
			ConditionalTypeConsequence::UnionType(it) => &it.syntax,
			ConditionalTypeConsequence::PrimaryType(it) => &it.syntax(),
		}
	}
}
impl From<PrimaryType> for ConditionalTypeLeft {
	fn from(node: PrimaryType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::PrimaryType(node)
	}
}
impl From<ConstructorType> for ConditionalTypeLeft {
	fn from(node: ConstructorType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::ConstructorType(node)
	}
}
impl From<FunctionType> for ConditionalTypeLeft {
	fn from(node: FunctionType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::FunctionType(node)
	}
}
impl From<InferType> for ConditionalTypeLeft {
	fn from(node: InferType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::InferType(node)
	}
}
impl From<IntersectionType> for ConditionalTypeLeft {
	fn from(node: IntersectionType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::IntersectionType(node)
	}
}
impl From<ReadonlyType> for ConditionalTypeLeft {
	fn from(node: ReadonlyType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::ReadonlyType(node)
	}
}
impl From<UnionType> for ConditionalTypeLeft {
	fn from(node: UnionType) -> ConditionalTypeLeft {
		ConditionalTypeLeft::UnionType(node)
	}
}
impl AstNode for ConditionalTypeLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ConstructorType
			| SyntaxKind::FunctionType
			| SyntaxKind::InferType
			| SyntaxKind::IntersectionType
			| SyntaxKind::ReadonlyType
			| SyntaxKind::UnionType => true,
			_ => PrimaryType::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ConstructorType => {
				ConditionalTypeLeft::ConstructorType(ConstructorType { syntax })
			}
			SyntaxKind::FunctionType => ConditionalTypeLeft::FunctionType(FunctionType { syntax }),
			SyntaxKind::InferType => ConditionalTypeLeft::InferType(InferType { syntax }),
			SyntaxKind::IntersectionType => {
				ConditionalTypeLeft::IntersectionType(IntersectionType { syntax })
			}
			SyntaxKind::ReadonlyType => ConditionalTypeLeft::ReadonlyType(ReadonlyType { syntax }),
			SyntaxKind::UnionType => ConditionalTypeLeft::UnionType(UnionType { syntax }),
			kind if PrimaryType::can_cast(kind) => {
				ConditionalTypeLeft::PrimaryType(PrimaryType::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConditionalTypeLeft::ConstructorType(it) => &it.syntax,
			ConditionalTypeLeft::FunctionType(it) => &it.syntax,
			ConditionalTypeLeft::InferType(it) => &it.syntax,
			ConditionalTypeLeft::IntersectionType(it) => &it.syntax,
			ConditionalTypeLeft::ReadonlyType(it) => &it.syntax,
			ConditionalTypeLeft::UnionType(it) => &it.syntax,
			ConditionalTypeLeft::PrimaryType(it) => &it.syntax(),
		}
	}
}
impl From<PrimaryType> for ConditionalTypeRight {
	fn from(node: PrimaryType) -> ConditionalTypeRight {
		ConditionalTypeRight::PrimaryType(node)
	}
}
impl From<ConstructorType> for ConditionalTypeRight {
	fn from(node: ConstructorType) -> ConditionalTypeRight {
		ConditionalTypeRight::ConstructorType(node)
	}
}
impl From<FunctionType> for ConditionalTypeRight {
	fn from(node: FunctionType) -> ConditionalTypeRight {
		ConditionalTypeRight::FunctionType(node)
	}
}
impl From<InferType> for ConditionalTypeRight {
	fn from(node: InferType) -> ConditionalTypeRight {
		ConditionalTypeRight::InferType(node)
	}
}
impl From<IntersectionType> for ConditionalTypeRight {
	fn from(node: IntersectionType) -> ConditionalTypeRight {
		ConditionalTypeRight::IntersectionType(node)
	}
}
impl From<ReadonlyType> for ConditionalTypeRight {
	fn from(node: ReadonlyType) -> ConditionalTypeRight {
		ConditionalTypeRight::ReadonlyType(node)
	}
}
impl From<UnionType> for ConditionalTypeRight {
	fn from(node: UnionType) -> ConditionalTypeRight {
		ConditionalTypeRight::UnionType(node)
	}
}
impl AstNode for ConditionalTypeRight {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ConstructorType
			| SyntaxKind::FunctionType
			| SyntaxKind::InferType
			| SyntaxKind::IntersectionType
			| SyntaxKind::ReadonlyType
			| SyntaxKind::UnionType => true,
			_ => PrimaryType::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ConstructorType => {
				ConditionalTypeRight::ConstructorType(ConstructorType { syntax })
			}
			SyntaxKind::FunctionType => ConditionalTypeRight::FunctionType(FunctionType { syntax }),
			SyntaxKind::InferType => ConditionalTypeRight::InferType(InferType { syntax }),
			SyntaxKind::IntersectionType => {
				ConditionalTypeRight::IntersectionType(IntersectionType { syntax })
			}
			SyntaxKind::ReadonlyType => ConditionalTypeRight::ReadonlyType(ReadonlyType { syntax }),
			SyntaxKind::UnionType => ConditionalTypeRight::UnionType(UnionType { syntax }),
			kind if PrimaryType::can_cast(kind) => {
				ConditionalTypeRight::PrimaryType(PrimaryType::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConditionalTypeRight::ConstructorType(it) => &it.syntax,
			ConditionalTypeRight::FunctionType(it) => &it.syntax,
			ConditionalTypeRight::InferType(it) => &it.syntax,
			ConditionalTypeRight::IntersectionType(it) => &it.syntax,
			ConditionalTypeRight::ReadonlyType(it) => &it.syntax,
			ConditionalTypeRight::UnionType(it) => &it.syntax,
			ConditionalTypeRight::PrimaryType(it) => &it.syntax(),
		}
	}
}
impl From<ArrayPattern> for ForInStatementLeft {
	fn from(node: ArrayPattern) -> ForInStatementLeft {
		ForInStatementLeft::ArrayPattern(node)
	}
}
impl From<Identifier> for ForInStatementLeft {
	fn from(node: Identifier) -> ForInStatementLeft {
		ForInStatementLeft::Identifier(node)
	}
}
impl From<MemberExpression> for ForInStatementLeft {
	fn from(node: MemberExpression) -> ForInStatementLeft {
		ForInStatementLeft::MemberExpression(node)
	}
}
impl From<NonNullExpression> for ForInStatementLeft {
	fn from(node: NonNullExpression) -> ForInStatementLeft {
		ForInStatementLeft::NonNullExpression(node)
	}
}
impl From<ObjectPattern> for ForInStatementLeft {
	fn from(node: ObjectPattern) -> ForInStatementLeft {
		ForInStatementLeft::ObjectPattern(node)
	}
}
impl From<ParenthesizedExpression> for ForInStatementLeft {
	fn from(node: ParenthesizedExpression) -> ForInStatementLeft {
		ForInStatementLeft::ParenthesizedExpression(node)
	}
}
impl From<SubscriptExpression> for ForInStatementLeft {
	fn from(node: SubscriptExpression) -> ForInStatementLeft {
		ForInStatementLeft::SubscriptExpression(node)
	}
}
impl AstNode for ForInStatementLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern
			| SyntaxKind::Identifier
			| SyntaxKind::MemberExpression
			| SyntaxKind::NonNullExpression
			| SyntaxKind::ObjectPattern
			| SyntaxKind::ParenthesizedExpression
			| SyntaxKind::SubscriptExpression => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => ForInStatementLeft::ArrayPattern(ArrayPattern { syntax }),
			SyntaxKind::Identifier => ForInStatementLeft::Identifier(Identifier { syntax }),
			SyntaxKind::MemberExpression => {
				ForInStatementLeft::MemberExpression(MemberExpression { syntax })
			}
			SyntaxKind::NonNullExpression => {
				ForInStatementLeft::NonNullExpression(NonNullExpression { syntax })
			}
			SyntaxKind::ObjectPattern => {
				ForInStatementLeft::ObjectPattern(ObjectPattern { syntax })
			}
			SyntaxKind::ParenthesizedExpression => {
				ForInStatementLeft::ParenthesizedExpression(ParenthesizedExpression { syntax })
			}
			SyntaxKind::SubscriptExpression => {
				ForInStatementLeft::SubscriptExpression(SubscriptExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForInStatementLeft::ArrayPattern(it) => &it.syntax,
			ForInStatementLeft::Identifier(it) => &it.syntax,
			ForInStatementLeft::MemberExpression(it) => &it.syntax,
			ForInStatementLeft::NonNullExpression(it) => &it.syntax,
			ForInStatementLeft::ObjectPattern(it) => &it.syntax,
			ForInStatementLeft::ParenthesizedExpression(it) => &it.syntax,
			ForInStatementLeft::SubscriptExpression(it) => &it.syntax,
		}
	}
}
impl From<Expression> for ForInStatementRight {
	fn from(node: Expression) -> ForInStatementRight {
		ForInStatementRight::Expression(node)
	}
}
impl From<SequenceExpression> for ForInStatementRight {
	fn from(node: SequenceExpression) -> ForInStatementRight {
		ForInStatementRight::SequenceExpression(node)
	}
}
impl AstNode for ForInStatementRight {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::SequenceExpression => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::SequenceExpression => {
				ForInStatementRight::SequenceExpression(SequenceExpression { syntax })
			}
			kind if Expression::can_cast(kind) => {
				ForInStatementRight::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForInStatementRight::SequenceExpression(it) => &it.syntax,
			ForInStatementRight::Expression(it) => &it.syntax(),
		}
	}
}
impl From<EmptyStatement> for ForStatementCondition {
	fn from(node: EmptyStatement) -> ForStatementCondition {
		ForStatementCondition::EmptyStatement(node)
	}
}
impl From<ExpressionStatement> for ForStatementCondition {
	fn from(node: ExpressionStatement) -> ForStatementCondition {
		ForStatementCondition::ExpressionStatement(node)
	}
}
impl AstNode for ForStatementCondition {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::EmptyStatement | SyntaxKind::ExpressionStatement => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::EmptyStatement => {
				ForStatementCondition::EmptyStatement(EmptyStatement { syntax })
			}
			SyntaxKind::ExpressionStatement => {
				ForStatementCondition::ExpressionStatement(ExpressionStatement { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForStatementCondition::EmptyStatement(it) => &it.syntax,
			ForStatementCondition::ExpressionStatement(it) => &it.syntax,
		}
	}
}
impl From<Expression> for ForStatementIncrement {
	fn from(node: Expression) -> ForStatementIncrement {
		ForStatementIncrement::Expression(node)
	}
}
impl From<SequenceExpression> for ForStatementIncrement {
	fn from(node: SequenceExpression) -> ForStatementIncrement {
		ForStatementIncrement::SequenceExpression(node)
	}
}
impl AstNode for ForStatementIncrement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::SequenceExpression => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::SequenceExpression => {
				ForStatementIncrement::SequenceExpression(SequenceExpression { syntax })
			}
			kind if Expression::can_cast(kind) => {
				ForStatementIncrement::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForStatementIncrement::SequenceExpression(it) => &it.syntax,
			ForStatementIncrement::Expression(it) => &it.syntax(),
		}
	}
}
impl From<EmptyStatement> for ForStatementInitializer {
	fn from(node: EmptyStatement) -> ForStatementInitializer {
		ForStatementInitializer::EmptyStatement(node)
	}
}
impl From<ExpressionStatement> for ForStatementInitializer {
	fn from(node: ExpressionStatement) -> ForStatementInitializer {
		ForStatementInitializer::ExpressionStatement(node)
	}
}
impl From<LexicalDeclaration> for ForStatementInitializer {
	fn from(node: LexicalDeclaration) -> ForStatementInitializer {
		ForStatementInitializer::LexicalDeclaration(node)
	}
}
impl From<VariableDeclaration> for ForStatementInitializer {
	fn from(node: VariableDeclaration) -> ForStatementInitializer {
		ForStatementInitializer::VariableDeclaration(node)
	}
}
impl AstNode for ForStatementInitializer {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::EmptyStatement
			| SyntaxKind::ExpressionStatement
			| SyntaxKind::LexicalDeclaration
			| SyntaxKind::VariableDeclaration => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::EmptyStatement => {
				ForStatementInitializer::EmptyStatement(EmptyStatement { syntax })
			}
			SyntaxKind::ExpressionStatement => {
				ForStatementInitializer::ExpressionStatement(ExpressionStatement { syntax })
			}
			SyntaxKind::LexicalDeclaration => {
				ForStatementInitializer::LexicalDeclaration(LexicalDeclaration { syntax })
			}
			SyntaxKind::VariableDeclaration => {
				ForStatementInitializer::VariableDeclaration(VariableDeclaration { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForStatementInitializer::EmptyStatement(it) => &it.syntax,
			ForStatementInitializer::ExpressionStatement(it) => &it.syntax,
			ForStatementInitializer::LexicalDeclaration(it) => &it.syntax,
			ForStatementInitializer::VariableDeclaration(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for FunctionReturnType {
	fn from(node: Asserts) -> FunctionReturnType {
		FunctionReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for FunctionReturnType {
	fn from(node: TypeAnnotation) -> FunctionReturnType {
		FunctionReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for FunctionReturnType {
	fn from(node: TypePredicateAnnotation) -> FunctionReturnType {
		FunctionReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for FunctionReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => FunctionReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				FunctionReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				FunctionReturnType::TypePredicateAnnotation(TypePredicateAnnotation { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			FunctionReturnType::Asserts(it) => &it.syntax,
			FunctionReturnType::TypeAnnotation(it) => &it.syntax,
			FunctionReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for FunctionDeclarationReturnType {
	fn from(node: Asserts) -> FunctionDeclarationReturnType {
		FunctionDeclarationReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for FunctionDeclarationReturnType {
	fn from(node: TypeAnnotation) -> FunctionDeclarationReturnType {
		FunctionDeclarationReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for FunctionDeclarationReturnType {
	fn from(node: TypePredicateAnnotation) -> FunctionDeclarationReturnType {
		FunctionDeclarationReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for FunctionDeclarationReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => FunctionDeclarationReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				FunctionDeclarationReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				FunctionDeclarationReturnType::TypePredicateAnnotation(TypePredicateAnnotation {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			FunctionDeclarationReturnType::Asserts(it) => &it.syntax,
			FunctionDeclarationReturnType::TypeAnnotation(it) => &it.syntax,
			FunctionDeclarationReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for FunctionSignatureReturnType {
	fn from(node: Asserts) -> FunctionSignatureReturnType {
		FunctionSignatureReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for FunctionSignatureReturnType {
	fn from(node: TypeAnnotation) -> FunctionSignatureReturnType {
		FunctionSignatureReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for FunctionSignatureReturnType {
	fn from(node: TypePredicateAnnotation) -> FunctionSignatureReturnType {
		FunctionSignatureReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for FunctionSignatureReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => FunctionSignatureReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				FunctionSignatureReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				FunctionSignatureReturnType::TypePredicateAnnotation(TypePredicateAnnotation {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			FunctionSignatureReturnType::Asserts(it) => &it.syntax,
			FunctionSignatureReturnType::TypeAnnotation(it) => &it.syntax,
			FunctionSignatureReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for GeneratorFunctionReturnType {
	fn from(node: Asserts) -> GeneratorFunctionReturnType {
		GeneratorFunctionReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for GeneratorFunctionReturnType {
	fn from(node: TypeAnnotation) -> GeneratorFunctionReturnType {
		GeneratorFunctionReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for GeneratorFunctionReturnType {
	fn from(node: TypePredicateAnnotation) -> GeneratorFunctionReturnType {
		GeneratorFunctionReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for GeneratorFunctionReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => GeneratorFunctionReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				GeneratorFunctionReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				GeneratorFunctionReturnType::TypePredicateAnnotation(TypePredicateAnnotation {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			GeneratorFunctionReturnType::Asserts(it) => &it.syntax,
			GeneratorFunctionReturnType::TypeAnnotation(it) => &it.syntax,
			GeneratorFunctionReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for GeneratorFunctionDeclarationReturnType {
	fn from(node: Asserts) -> GeneratorFunctionDeclarationReturnType {
		GeneratorFunctionDeclarationReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for GeneratorFunctionDeclarationReturnType {
	fn from(node: TypeAnnotation) -> GeneratorFunctionDeclarationReturnType {
		GeneratorFunctionDeclarationReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for GeneratorFunctionDeclarationReturnType {
	fn from(node: TypePredicateAnnotation) -> GeneratorFunctionDeclarationReturnType {
		GeneratorFunctionDeclarationReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for GeneratorFunctionDeclarationReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => {
				GeneratorFunctionDeclarationReturnType::Asserts(Asserts { syntax })
			}
			SyntaxKind::TypeAnnotation => {
				GeneratorFunctionDeclarationReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				GeneratorFunctionDeclarationReturnType::TypePredicateAnnotation(
					TypePredicateAnnotation { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			GeneratorFunctionDeclarationReturnType::Asserts(it) => &it.syntax,
			GeneratorFunctionDeclarationReturnType::TypeAnnotation(it) => &it.syntax,
			GeneratorFunctionDeclarationReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for InternalModuleName {
	fn from(node: Identifier) -> InternalModuleName {
		InternalModuleName::Identifier(node)
	}
}
impl From<NestedIdentifier> for InternalModuleName {
	fn from(node: NestedIdentifier) -> InternalModuleName {
		InternalModuleName::NestedIdentifier(node)
	}
}
impl From<StringLiteral> for InternalModuleName {
	fn from(node: StringLiteral) -> InternalModuleName {
		InternalModuleName::StringLiteral(node)
	}
}
impl AstNode for InternalModuleName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier | SyntaxKind::NestedIdentifier | SyntaxKind::StringLiteral => {
				true
			}
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => InternalModuleName::Identifier(Identifier { syntax }),
			SyntaxKind::NestedIdentifier => {
				InternalModuleName::NestedIdentifier(NestedIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				InternalModuleName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			InternalModuleName::Identifier(it) => &it.syntax,
			InternalModuleName::NestedIdentifier(it) => &it.syntax,
			InternalModuleName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for JsxClosingElementName {
	fn from(node: Identifier) -> JsxClosingElementName {
		JsxClosingElementName::Identifier(node)
	}
}
impl From<JsxNamespaceName> for JsxClosingElementName {
	fn from(node: JsxNamespaceName) -> JsxClosingElementName {
		JsxClosingElementName::JsxNamespaceName(node)
	}
}
impl From<NestedIdentifier> for JsxClosingElementName {
	fn from(node: NestedIdentifier) -> JsxClosingElementName {
		JsxClosingElementName::NestedIdentifier(node)
	}
}
impl AstNode for JsxClosingElementName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier
			| SyntaxKind::JsxNamespaceName
			| SyntaxKind::NestedIdentifier => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => JsxClosingElementName::Identifier(Identifier { syntax }),
			SyntaxKind::JsxNamespaceName => {
				JsxClosingElementName::JsxNamespaceName(JsxNamespaceName { syntax })
			}
			SyntaxKind::NestedIdentifier => {
				JsxClosingElementName::NestedIdentifier(NestedIdentifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsxClosingElementName::Identifier(it) => &it.syntax,
			JsxClosingElementName::JsxNamespaceName(it) => &it.syntax,
			JsxClosingElementName::NestedIdentifier(it) => &it.syntax,
		}
	}
}
impl From<JsxAttribute> for JsxOpeningElementAttribute {
	fn from(node: JsxAttribute) -> JsxOpeningElementAttribute {
		JsxOpeningElementAttribute::JsxAttribute(node)
	}
}
impl From<JsxExpression> for JsxOpeningElementAttribute {
	fn from(node: JsxExpression) -> JsxOpeningElementAttribute {
		JsxOpeningElementAttribute::JsxExpression(node)
	}
}
impl AstNode for JsxOpeningElementAttribute {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::JsxAttribute | SyntaxKind::JsxExpression => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::JsxAttribute => {
				JsxOpeningElementAttribute::JsxAttribute(JsxAttribute { syntax })
			}
			SyntaxKind::JsxExpression => {
				JsxOpeningElementAttribute::JsxExpression(JsxExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsxOpeningElementAttribute::JsxAttribute(it) => &it.syntax,
			JsxOpeningElementAttribute::JsxExpression(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for JsxOpeningElementName {
	fn from(node: Identifier) -> JsxOpeningElementName {
		JsxOpeningElementName::Identifier(node)
	}
}
impl From<JsxNamespaceName> for JsxOpeningElementName {
	fn from(node: JsxNamespaceName) -> JsxOpeningElementName {
		JsxOpeningElementName::JsxNamespaceName(node)
	}
}
impl From<NestedIdentifier> for JsxOpeningElementName {
	fn from(node: NestedIdentifier) -> JsxOpeningElementName {
		JsxOpeningElementName::NestedIdentifier(node)
	}
}
impl AstNode for JsxOpeningElementName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier
			| SyntaxKind::JsxNamespaceName
			| SyntaxKind::NestedIdentifier => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => JsxOpeningElementName::Identifier(Identifier { syntax }),
			SyntaxKind::JsxNamespaceName => {
				JsxOpeningElementName::JsxNamespaceName(JsxNamespaceName { syntax })
			}
			SyntaxKind::NestedIdentifier => {
				JsxOpeningElementName::NestedIdentifier(NestedIdentifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsxOpeningElementName::Identifier(it) => &it.syntax,
			JsxOpeningElementName::JsxNamespaceName(it) => &it.syntax,
			JsxOpeningElementName::NestedIdentifier(it) => &it.syntax,
		}
	}
}
impl From<JsxAttribute> for JsxSelfClosingElementAttribute {
	fn from(node: JsxAttribute) -> JsxSelfClosingElementAttribute {
		JsxSelfClosingElementAttribute::JsxAttribute(node)
	}
}
impl From<JsxExpression> for JsxSelfClosingElementAttribute {
	fn from(node: JsxExpression) -> JsxSelfClosingElementAttribute {
		JsxSelfClosingElementAttribute::JsxExpression(node)
	}
}
impl AstNode for JsxSelfClosingElementAttribute {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::JsxAttribute | SyntaxKind::JsxExpression => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::JsxAttribute => {
				JsxSelfClosingElementAttribute::JsxAttribute(JsxAttribute { syntax })
			}
			SyntaxKind::JsxExpression => {
				JsxSelfClosingElementAttribute::JsxExpression(JsxExpression { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsxSelfClosingElementAttribute::JsxAttribute(it) => &it.syntax,
			JsxSelfClosingElementAttribute::JsxExpression(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for JsxSelfClosingElementName {
	fn from(node: Identifier) -> JsxSelfClosingElementName {
		JsxSelfClosingElementName::Identifier(node)
	}
}
impl From<JsxNamespaceName> for JsxSelfClosingElementName {
	fn from(node: JsxNamespaceName) -> JsxSelfClosingElementName {
		JsxSelfClosingElementName::JsxNamespaceName(node)
	}
}
impl From<NestedIdentifier> for JsxSelfClosingElementName {
	fn from(node: NestedIdentifier) -> JsxSelfClosingElementName {
		JsxSelfClosingElementName::NestedIdentifier(node)
	}
}
impl AstNode for JsxSelfClosingElementName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier
			| SyntaxKind::JsxNamespaceName
			| SyntaxKind::NestedIdentifier => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => JsxSelfClosingElementName::Identifier(Identifier { syntax }),
			SyntaxKind::JsxNamespaceName => {
				JsxSelfClosingElementName::JsxNamespaceName(JsxNamespaceName { syntax })
			}
			SyntaxKind::NestedIdentifier => {
				JsxSelfClosingElementName::NestedIdentifier(NestedIdentifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsxSelfClosingElementName::Identifier(it) => &it.syntax,
			JsxSelfClosingElementName::JsxNamespaceName(it) => &it.syntax,
			JsxSelfClosingElementName::NestedIdentifier(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for MethodDefinitionName {
	fn from(node: ComputedPropertyName) -> MethodDefinitionName {
		MethodDefinitionName::ComputedPropertyName(node)
	}
}
impl From<Number> for MethodDefinitionName {
	fn from(node: Number) -> MethodDefinitionName {
		MethodDefinitionName::Number(node)
	}
}
impl From<PropertyIdentifier> for MethodDefinitionName {
	fn from(node: PropertyIdentifier) -> MethodDefinitionName {
		MethodDefinitionName::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for MethodDefinitionName {
	fn from(node: StringLiteral) -> MethodDefinitionName {
		MethodDefinitionName::StringLiteral(node)
	}
}
impl AstNode for MethodDefinitionName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				MethodDefinitionName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => MethodDefinitionName::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				MethodDefinitionName::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				MethodDefinitionName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			MethodDefinitionName::ComputedPropertyName(it) => &it.syntax,
			MethodDefinitionName::Number(it) => &it.syntax,
			MethodDefinitionName::PropertyIdentifier(it) => &it.syntax,
			MethodDefinitionName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for MethodDefinitionReturnType {
	fn from(node: Asserts) -> MethodDefinitionReturnType {
		MethodDefinitionReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for MethodDefinitionReturnType {
	fn from(node: TypeAnnotation) -> MethodDefinitionReturnType {
		MethodDefinitionReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for MethodDefinitionReturnType {
	fn from(node: TypePredicateAnnotation) -> MethodDefinitionReturnType {
		MethodDefinitionReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for MethodDefinitionReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => MethodDefinitionReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				MethodDefinitionReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				MethodDefinitionReturnType::TypePredicateAnnotation(TypePredicateAnnotation {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			MethodDefinitionReturnType::Asserts(it) => &it.syntax,
			MethodDefinitionReturnType::TypeAnnotation(it) => &it.syntax,
			MethodDefinitionReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for MethodSignatureName {
	fn from(node: ComputedPropertyName) -> MethodSignatureName {
		MethodSignatureName::ComputedPropertyName(node)
	}
}
impl From<Number> for MethodSignatureName {
	fn from(node: Number) -> MethodSignatureName {
		MethodSignatureName::Number(node)
	}
}
impl From<PropertyIdentifier> for MethodSignatureName {
	fn from(node: PropertyIdentifier) -> MethodSignatureName {
		MethodSignatureName::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for MethodSignatureName {
	fn from(node: StringLiteral) -> MethodSignatureName {
		MethodSignatureName::StringLiteral(node)
	}
}
impl AstNode for MethodSignatureName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				MethodSignatureName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => MethodSignatureName::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				MethodSignatureName::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				MethodSignatureName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			MethodSignatureName::ComputedPropertyName(it) => &it.syntax,
			MethodSignatureName::Number(it) => &it.syntax,
			MethodSignatureName::PropertyIdentifier(it) => &it.syntax,
			MethodSignatureName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Asserts> for MethodSignatureReturnType {
	fn from(node: Asserts) -> MethodSignatureReturnType {
		MethodSignatureReturnType::Asserts(node)
	}
}
impl From<TypeAnnotation> for MethodSignatureReturnType {
	fn from(node: TypeAnnotation) -> MethodSignatureReturnType {
		MethodSignatureReturnType::TypeAnnotation(node)
	}
}
impl From<TypePredicateAnnotation> for MethodSignatureReturnType {
	fn from(node: TypePredicateAnnotation) -> MethodSignatureReturnType {
		MethodSignatureReturnType::TypePredicateAnnotation(node)
	}
}
impl AstNode for MethodSignatureReturnType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Asserts
			| SyntaxKind::TypeAnnotation
			| SyntaxKind::TypePredicateAnnotation => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Asserts => MethodSignatureReturnType::Asserts(Asserts { syntax }),
			SyntaxKind::TypeAnnotation => {
				MethodSignatureReturnType::TypeAnnotation(TypeAnnotation { syntax })
			}
			SyntaxKind::TypePredicateAnnotation => {
				MethodSignatureReturnType::TypePredicateAnnotation(TypePredicateAnnotation {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			MethodSignatureReturnType::Asserts(it) => &it.syntax,
			MethodSignatureReturnType::TypeAnnotation(it) => &it.syntax,
			MethodSignatureReturnType::TypePredicateAnnotation(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for ModuleName {
	fn from(node: Identifier) -> ModuleName {
		ModuleName::Identifier(node)
	}
}
impl From<NestedIdentifier> for ModuleName {
	fn from(node: NestedIdentifier) -> ModuleName {
		ModuleName::NestedIdentifier(node)
	}
}
impl From<StringLiteral> for ModuleName {
	fn from(node: StringLiteral) -> ModuleName {
		ModuleName::StringLiteral(node)
	}
}
impl AstNode for ModuleName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier | SyntaxKind::NestedIdentifier | SyntaxKind::StringLiteral => {
				true
			}
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => ModuleName::Identifier(Identifier { syntax }),
			SyntaxKind::NestedIdentifier => {
				ModuleName::NestedIdentifier(NestedIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => ModuleName::StringLiteral(StringLiteral { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ModuleName::Identifier(it) => &it.syntax,
			ModuleName::NestedIdentifier(it) => &it.syntax,
			ModuleName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Identifier> for NestedTypeIdentifierModule {
	fn from(node: Identifier) -> NestedTypeIdentifierModule {
		NestedTypeIdentifierModule::Identifier(node)
	}
}
impl From<NestedIdentifier> for NestedTypeIdentifierModule {
	fn from(node: NestedIdentifier) -> NestedTypeIdentifierModule {
		NestedTypeIdentifierModule::NestedIdentifier(node)
	}
}
impl AstNode for NestedTypeIdentifierModule {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Identifier | SyntaxKind::NestedIdentifier => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Identifier => NestedTypeIdentifierModule::Identifier(Identifier { syntax }),
			SyntaxKind::NestedIdentifier => {
				NestedTypeIdentifierModule::NestedIdentifier(NestedIdentifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			NestedTypeIdentifierModule::Identifier(it) => &it.syntax,
			NestedTypeIdentifierModule::NestedIdentifier(it) => &it.syntax,
		}
	}
}
impl From<ArrayPattern> for ObjectAssignmentPatternLeft {
	fn from(node: ArrayPattern) -> ObjectAssignmentPatternLeft {
		ObjectAssignmentPatternLeft::ArrayPattern(node)
	}
}
impl From<ObjectPattern> for ObjectAssignmentPatternLeft {
	fn from(node: ObjectPattern) -> ObjectAssignmentPatternLeft {
		ObjectAssignmentPatternLeft::ObjectPattern(node)
	}
}
impl From<ShorthandPropertyIdentifierPattern> for ObjectAssignmentPatternLeft {
	fn from(node: ShorthandPropertyIdentifierPattern) -> ObjectAssignmentPatternLeft {
		ObjectAssignmentPatternLeft::ShorthandPropertyIdentifierPattern(node)
	}
}
impl AstNode for ObjectAssignmentPatternLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern
			| SyntaxKind::ObjectPattern
			| SyntaxKind::ShorthandPropertyIdentifierPattern => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => {
				ObjectAssignmentPatternLeft::ArrayPattern(ArrayPattern { syntax })
			}
			SyntaxKind::ObjectPattern => {
				ObjectAssignmentPatternLeft::ObjectPattern(ObjectPattern { syntax })
			}
			SyntaxKind::ShorthandPropertyIdentifierPattern => {
				ObjectAssignmentPatternLeft::ShorthandPropertyIdentifierPattern(
					ShorthandPropertyIdentifierPattern { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ObjectAssignmentPatternLeft::ArrayPattern(it) => &it.syntax,
			ObjectAssignmentPatternLeft::ObjectPattern(it) => &it.syntax,
			ObjectAssignmentPatternLeft::ShorthandPropertyIdentifierPattern(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for PairKey {
	fn from(node: ComputedPropertyName) -> PairKey {
		PairKey::ComputedPropertyName(node)
	}
}
impl From<Number> for PairKey {
	fn from(node: Number) -> PairKey {
		PairKey::Number(node)
	}
}
impl From<PropertyIdentifier> for PairKey {
	fn from(node: PropertyIdentifier) -> PairKey {
		PairKey::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for PairKey {
	fn from(node: StringLiteral) -> PairKey {
		PairKey::StringLiteral(node)
	}
}
impl AstNode for PairKey {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				PairKey::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => PairKey::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				PairKey::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => PairKey::StringLiteral(StringLiteral { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PairKey::ComputedPropertyName(it) => &it.syntax,
			PairKey::Number(it) => &it.syntax,
			PairKey::PropertyIdentifier(it) => &it.syntax,
			PairKey::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for PairPatternKey {
	fn from(node: ComputedPropertyName) -> PairPatternKey {
		PairPatternKey::ComputedPropertyName(node)
	}
}
impl From<Number> for PairPatternKey {
	fn from(node: Number) -> PairPatternKey {
		PairPatternKey::Number(node)
	}
}
impl From<PropertyIdentifier> for PairPatternKey {
	fn from(node: PropertyIdentifier) -> PairPatternKey {
		PairPatternKey::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for PairPatternKey {
	fn from(node: StringLiteral) -> PairPatternKey {
		PairPatternKey::StringLiteral(node)
	}
}
impl AstNode for PairPatternKey {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				PairPatternKey::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => PairPatternKey::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				PairPatternKey::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => PairPatternKey::StringLiteral(StringLiteral { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PairPatternKey::ComputedPropertyName(it) => &it.syntax,
			PairPatternKey::Number(it) => &it.syntax,
			PairPatternKey::PropertyIdentifier(it) => &it.syntax,
			PairPatternKey::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for PropertySignatureName {
	fn from(node: ComputedPropertyName) -> PropertySignatureName {
		PropertySignatureName::ComputedPropertyName(node)
	}
}
impl From<Number> for PropertySignatureName {
	fn from(node: Number) -> PropertySignatureName {
		PropertySignatureName::Number(node)
	}
}
impl From<PropertyIdentifier> for PropertySignatureName {
	fn from(node: PropertyIdentifier) -> PropertySignatureName {
		PropertySignatureName::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for PropertySignatureName {
	fn from(node: StringLiteral) -> PropertySignatureName {
		PropertySignatureName::StringLiteral(node)
	}
}
impl AstNode for PropertySignatureName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				PropertySignatureName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => PropertySignatureName::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				PropertySignatureName::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				PropertySignatureName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PropertySignatureName::ComputedPropertyName(it) => &it.syntax,
			PropertySignatureName::Number(it) => &it.syntax,
			PropertySignatureName::PropertyIdentifier(it) => &it.syntax,
			PropertySignatureName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for PublicFieldDefinitionName {
	fn from(node: ComputedPropertyName) -> PublicFieldDefinitionName {
		PublicFieldDefinitionName::ComputedPropertyName(node)
	}
}
impl From<Number> for PublicFieldDefinitionName {
	fn from(node: Number) -> PublicFieldDefinitionName {
		PublicFieldDefinitionName::Number(node)
	}
}
impl From<PropertyIdentifier> for PublicFieldDefinitionName {
	fn from(node: PropertyIdentifier) -> PublicFieldDefinitionName {
		PublicFieldDefinitionName::PropertyIdentifier(node)
	}
}
impl From<StringLiteral> for PublicFieldDefinitionName {
	fn from(node: StringLiteral) -> PublicFieldDefinitionName {
		PublicFieldDefinitionName::StringLiteral(node)
	}
}
impl AstNode for PublicFieldDefinitionName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ComputedPropertyName
			| SyntaxKind::Number
			| SyntaxKind::PropertyIdentifier
			| SyntaxKind::StringLiteral => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ComputedPropertyName => {
				PublicFieldDefinitionName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			SyntaxKind::Number => PublicFieldDefinitionName::Number(Number { syntax }),
			SyntaxKind::PropertyIdentifier => {
				PublicFieldDefinitionName::PropertyIdentifier(PropertyIdentifier { syntax })
			}
			SyntaxKind::StringLiteral => {
				PublicFieldDefinitionName::StringLiteral(StringLiteral { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PublicFieldDefinitionName::ComputedPropertyName(it) => &it.syntax,
			PublicFieldDefinitionName::Number(it) => &it.syntax,
			PublicFieldDefinitionName::PropertyIdentifier(it) => &it.syntax,
			PublicFieldDefinitionName::StringLiteral(it) => &it.syntax,
		}
	}
}
impl From<Expression> for SequenceExpressionRight {
	fn from(node: Expression) -> SequenceExpressionRight {
		SequenceExpressionRight::Expression(node)
	}
}
impl From<SequenceExpression> for SequenceExpressionRight {
	fn from(node: SequenceExpression) -> SequenceExpressionRight {
		SequenceExpressionRight::SequenceExpression(node)
	}
}
impl AstNode for SequenceExpressionRight {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::SequenceExpression => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::SequenceExpression => {
				SequenceExpressionRight::SequenceExpression(SequenceExpression { syntax })
			}
			kind if Expression::can_cast(kind) => {
				SequenceExpressionRight::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			SequenceExpressionRight::SequenceExpression(it) => &it.syntax,
			SequenceExpressionRight::Expression(it) => &it.syntax(),
		}
	}
}
impl From<Expression> for SubscriptExpressionIndex {
	fn from(node: Expression) -> SubscriptExpressionIndex {
		SubscriptExpressionIndex::Expression(node)
	}
}
impl From<SequenceExpression> for SubscriptExpressionIndex {
	fn from(node: SequenceExpression) -> SubscriptExpressionIndex {
		SubscriptExpressionIndex::SequenceExpression(node)
	}
}
impl AstNode for SubscriptExpressionIndex {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::SequenceExpression => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::SequenceExpression => {
				SubscriptExpressionIndex::SequenceExpression(SequenceExpression { syntax })
			}
			kind if Expression::can_cast(kind) => {
				SubscriptExpressionIndex::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			SubscriptExpressionIndex::SequenceExpression(it) => &it.syntax,
			SubscriptExpressionIndex::Expression(it) => &it.syntax(),
		}
	}
}
impl From<Expression> for SwitchCaseValue {
	fn from(node: Expression) -> SwitchCaseValue {
		SwitchCaseValue::Expression(node)
	}
}
impl From<SequenceExpression> for SwitchCaseValue {
	fn from(node: SequenceExpression) -> SwitchCaseValue {
		SwitchCaseValue::SequenceExpression(node)
	}
}
impl AstNode for SwitchCaseValue {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::SequenceExpression => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::SequenceExpression => {
				SwitchCaseValue::SequenceExpression(SequenceExpression { syntax })
			}
			kind if Expression::can_cast(kind) => {
				SwitchCaseValue::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			SwitchCaseValue::SequenceExpression(it) => &it.syntax,
			SwitchCaseValue::Expression(it) => &it.syntax(),
		}
	}
}
impl From<PrimaryType> for TypeAliasDeclarationValue {
	fn from(node: PrimaryType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::PrimaryType(node)
	}
}
impl From<ConstructorType> for TypeAliasDeclarationValue {
	fn from(node: ConstructorType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::ConstructorType(node)
	}
}
impl From<FunctionType> for TypeAliasDeclarationValue {
	fn from(node: FunctionType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::FunctionType(node)
	}
}
impl From<InferType> for TypeAliasDeclarationValue {
	fn from(node: InferType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::InferType(node)
	}
}
impl From<IntersectionType> for TypeAliasDeclarationValue {
	fn from(node: IntersectionType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::IntersectionType(node)
	}
}
impl From<ReadonlyType> for TypeAliasDeclarationValue {
	fn from(node: ReadonlyType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::ReadonlyType(node)
	}
}
impl From<UnionType> for TypeAliasDeclarationValue {
	fn from(node: UnionType) -> TypeAliasDeclarationValue {
		TypeAliasDeclarationValue::UnionType(node)
	}
}
impl AstNode for TypeAliasDeclarationValue {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ConstructorType
			| SyntaxKind::FunctionType
			| SyntaxKind::InferType
			| SyntaxKind::IntersectionType
			| SyntaxKind::ReadonlyType
			| SyntaxKind::UnionType => true,
			_ => PrimaryType::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ConstructorType => {
				TypeAliasDeclarationValue::ConstructorType(ConstructorType { syntax })
			}
			SyntaxKind::FunctionType => {
				TypeAliasDeclarationValue::FunctionType(FunctionType { syntax })
			}
			SyntaxKind::InferType => TypeAliasDeclarationValue::InferType(InferType { syntax }),
			SyntaxKind::IntersectionType => {
				TypeAliasDeclarationValue::IntersectionType(IntersectionType { syntax })
			}
			SyntaxKind::ReadonlyType => {
				TypeAliasDeclarationValue::ReadonlyType(ReadonlyType { syntax })
			}
			SyntaxKind::UnionType => TypeAliasDeclarationValue::UnionType(UnionType { syntax }),
			kind if PrimaryType::can_cast(kind) => {
				TypeAliasDeclarationValue::PrimaryType(PrimaryType::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TypeAliasDeclarationValue::ConstructorType(it) => &it.syntax,
			TypeAliasDeclarationValue::FunctionType(it) => &it.syntax,
			TypeAliasDeclarationValue::InferType(it) => &it.syntax,
			TypeAliasDeclarationValue::IntersectionType(it) => &it.syntax,
			TypeAliasDeclarationValue::ReadonlyType(it) => &it.syntax,
			TypeAliasDeclarationValue::UnionType(it) => &it.syntax,
			TypeAliasDeclarationValue::PrimaryType(it) => &it.syntax(),
		}
	}
}
impl From<Expression> for UnaryExpressionArgument {
	fn from(node: Expression) -> UnaryExpressionArgument {
		UnaryExpressionArgument::Expression(node)
	}
}
impl From<Number> for UnaryExpressionArgument {
	fn from(node: Number) -> UnaryExpressionArgument {
		UnaryExpressionArgument::Number(node)
	}
}
impl AstNode for UnaryExpressionArgument {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::Number => true,
			_ => Expression::can_cast(kind),
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::Number => UnaryExpressionArgument::Number(Number { syntax }),
			kind if Expression::can_cast(kind) => {
				UnaryExpressionArgument::Expression(Expression::cast(syntax)?)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			UnaryExpressionArgument::Number(it) => &it.syntax,
			UnaryExpressionArgument::Expression(it) => &it.syntax(),
		}
	}
}
impl From<BangToken> for UnaryExpressionOperator {
	fn from(node: BangToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::BangToken(node)
	}
}
impl From<PlusToken> for UnaryExpressionOperator {
	fn from(node: PlusToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::PlusToken(node)
	}
}
impl From<DashToken> for UnaryExpressionOperator {
	fn from(node: DashToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::DashToken(node)
	}
}
impl From<DeleteToken> for UnaryExpressionOperator {
	fn from(node: DeleteToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::DeleteToken(node)
	}
}
impl From<TypeofToken> for UnaryExpressionOperator {
	fn from(node: TypeofToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::TypeofToken(node)
	}
}
impl From<VoidToken> for UnaryExpressionOperator {
	fn from(node: VoidToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::VoidToken(node)
	}
}
impl From<TildeToken> for UnaryExpressionOperator {
	fn from(node: TildeToken) -> UnaryExpressionOperator {
		UnaryExpressionOperator::TildeToken(node)
	}
}
impl AstNode for UnaryExpressionOperator {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::BangToken
			| SyntaxKind::PlusToken
			| SyntaxKind::DashToken
			| SyntaxKind::DeleteToken
			| SyntaxKind::TypeofToken
			| SyntaxKind::VoidToken
			| SyntaxKind::TildeToken => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::BangToken => UnaryExpressionOperator::BangToken(BangToken { syntax }),
			SyntaxKind::PlusToken => UnaryExpressionOperator::PlusToken(PlusToken { syntax }),
			SyntaxKind::DashToken => UnaryExpressionOperator::DashToken(DashToken { syntax }),
			SyntaxKind::DeleteToken => UnaryExpressionOperator::DeleteToken(DeleteToken { syntax }),
			SyntaxKind::TypeofToken => UnaryExpressionOperator::TypeofToken(TypeofToken { syntax }),
			SyntaxKind::VoidToken => UnaryExpressionOperator::VoidToken(VoidToken { syntax }),
			SyntaxKind::TildeToken => UnaryExpressionOperator::TildeToken(TildeToken { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			UnaryExpressionOperator::BangToken(it) => &it.syntax,
			UnaryExpressionOperator::PlusToken(it) => &it.syntax,
			UnaryExpressionOperator::DashToken(it) => &it.syntax,
			UnaryExpressionOperator::DeleteToken(it) => &it.syntax,
			UnaryExpressionOperator::TypeofToken(it) => &it.syntax,
			UnaryExpressionOperator::VoidToken(it) => &it.syntax,
			UnaryExpressionOperator::TildeToken(it) => &it.syntax,
		}
	}
}
impl From<PlusPlusToken> for UpdateExpressionOperator {
	fn from(node: PlusPlusToken) -> UpdateExpressionOperator {
		UpdateExpressionOperator::PlusPlusToken(node)
	}
}
impl From<DashDashToken> for UpdateExpressionOperator {
	fn from(node: DashDashToken) -> UpdateExpressionOperator {
		UpdateExpressionOperator::DashDashToken(node)
	}
}
impl AstNode for UpdateExpressionOperator {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::PlusPlusToken | SyntaxKind::DashDashToken => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::PlusPlusToken => {
				UpdateExpressionOperator::PlusPlusToken(PlusPlusToken { syntax })
			}
			SyntaxKind::DashDashToken => {
				UpdateExpressionOperator::DashDashToken(DashDashToken { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			UpdateExpressionOperator::PlusPlusToken(it) => &it.syntax,
			UpdateExpressionOperator::DashDashToken(it) => &it.syntax,
		}
	}
}
impl From<ArrayPattern> for VariableDeclaratorName {
	fn from(node: ArrayPattern) -> VariableDeclaratorName {
		VariableDeclaratorName::ArrayPattern(node)
	}
}
impl From<Identifier> for VariableDeclaratorName {
	fn from(node: Identifier) -> VariableDeclaratorName {
		VariableDeclaratorName::Identifier(node)
	}
}
impl From<ObjectPattern> for VariableDeclaratorName {
	fn from(node: ObjectPattern) -> VariableDeclaratorName {
		VariableDeclaratorName::ObjectPattern(node)
	}
}
impl AstNode for VariableDeclaratorName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SyntaxKind::ArrayPattern | SyntaxKind::Identifier | SyntaxKind::ObjectPattern => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SyntaxKind::ArrayPattern => {
				VariableDeclaratorName::ArrayPattern(ArrayPattern { syntax })
			}
			SyntaxKind::Identifier => VariableDeclaratorName::Identifier(Identifier { syntax }),
			SyntaxKind::ObjectPattern => {
				VariableDeclaratorName::ObjectPattern(ObjectPattern { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			VariableDeclaratorName::ArrayPattern(it) => &it.syntax,
			VariableDeclaratorName::Identifier(it) => &it.syntax,
			VariableDeclaratorName::ObjectPattern(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for PrimaryType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Pattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrimaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AbstractMethodSignatureName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AbstractMethodSignatureReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowFunctionReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignmentExpressionLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AugmentedAssignmentExpressionLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BinaryExpressionOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpressionArguments {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallSignatureReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CatchClauseParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConditionalTypeAlternative {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConditionalTypeConsequence {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConditionalTypeLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConditionalTypeRight {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForInStatementLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForInStatementRight {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStatementCondition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStatementIncrement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStatementInitializer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionDeclarationReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionSignatureReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GeneratorFunctionReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GeneratorFunctionDeclarationReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InternalModuleName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxClosingElementName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxOpeningElementAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxOpeningElementName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxSelfClosingElementAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxSelfClosingElementName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodDefinitionName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodDefinitionReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodSignatureName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodSignatureReturnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ModuleName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NestedTypeIdentifierModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectAssignmentPatternLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PairKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PairPatternKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PropertySignatureName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PublicFieldDefinitionName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SequenceExpressionRight {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SubscriptExpressionIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchCaseValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeAliasDeclarationValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnaryExpressionArgument {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnaryExpressionOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UpdateExpressionOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VariableDeclaratorName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AbstractClassDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AbstractMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AccessibilityModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AmbientDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Arguments {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Array {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrayPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrayType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AsExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Asserts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AugmentedAssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AwaitExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BinaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BreakStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CatchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Class {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassHeritage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ComputedPropertyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConditionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Constraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConstructSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConstructorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ContinueStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DebuggerStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Decorator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DoStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ElseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EmptyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EnumAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EnumBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EnumDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExistentialType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExpressionStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExtendsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FinallyClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FlowMaybeType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForInStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FormalParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GeneratorFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GeneratorFunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GenericType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IfStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImplementsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Import {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportAlias {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportRequireClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IndexTypeQuery {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InferType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InterfaceDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InternalModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IntersectionType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxClosingElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxFragment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxNamespaceName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxOpeningElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxSelfClosingElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LabeledStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LexicalDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LiteralType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LookupType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MappedTypeClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MetaProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodDefinition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for MethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Module {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NamedImports {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NamespaceImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NestedIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NestedTypeIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NonNullExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for OmittingTypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for OptingTypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for OptionalParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for OptionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Pair {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PairPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ParenthesizedExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ParenthesizedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PredefinedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Program {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PublicFieldDefinition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ReadonlyType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Regex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RequiredParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RestPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RestType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ReturnStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SequenceExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SpreadElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StatementBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StringLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SubscriptExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchCase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TemplateString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TemplateSubstitution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TernaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ThrowStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TryStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TupleType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeAliasDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeArguments {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypePredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypePredicateAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeQuery {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnionType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VariableDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VariableDeclarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WithStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for YieldExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BangToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BangEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BangEqEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DquoteToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DollarLbraceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PercentToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PercentEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AmpToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AmpAmpToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AmpAmpEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AmpEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SquoteToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LparenToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RparenToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StarToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StarStarToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StarStarEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StarEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PlusToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PlusPlusToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PlusEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CommaToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DashToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DashDashToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DashEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DashQmarkColonToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DotToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DotDotDotToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SlashToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SlashEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ColonToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SemiToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LtLtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LtLtEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LtEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EqEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EqEqEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EqGtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtGtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtGtEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtGtGtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GtGtGtEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for QmarkToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for QmarkDotToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for QmarkColonToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for QmarkQmarkToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for QmarkQmarkEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AtToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LbrackToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RbrackToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CaretToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CaretEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BquoteToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AbstractToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AsToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssertsToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AsyncToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AwaitToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BooleanToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BreakToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CaseToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CatchToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConstToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ContinueToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DebuggerToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DeclareToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DeleteToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DoToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ElseToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EnumToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EscapeSequence {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExtendsToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for False {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FinallyToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FromToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FunctionToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GetToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GlobalToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for HashBangLine {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IfToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImplementsToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InferToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InstanceofToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InterfaceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IsToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsxText {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for KeyofToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LetToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ModuleToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NamespaceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Null {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NumberToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for OfToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivateToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PropertyIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ProtectedToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PublicToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ReadonlyToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RegexFlags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RegexPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RequireToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ReturnToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SetToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ShorthandPropertyIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ShorthandPropertyIdentifierPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StatementIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StaticToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StringToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Super {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SymbolToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TargetToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for This {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ThrowToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for True {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TryToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TypeofToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Undefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VarToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VoidToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WhileToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WithToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for YieldToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LbraceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LbracePipeToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PipeToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PipeEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PipePipeToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PipePipeEqToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PipeRbraceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RbraceToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TildeToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
