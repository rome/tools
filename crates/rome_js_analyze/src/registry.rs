//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{analyzers::*, assists::*};
use rome_analyze::{AnalysisFilter, AnalyzerSignal, ControlFlow, RuleRegistry};
use rome_js_syntax::JsLanguage;
pub(crate) fn build_registry<'a, F, B>(
    filter: &AnalysisFilter,
    callback: F,
) -> RuleRegistry<'a, JsLanguage, B>
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
{
    let mut rules = RuleRegistry::new(callback);
    if filter.match_rule::<NoAsyncPromiseExecutor>() {
        rules.push::<NoAsyncPromiseExecutor>();
    }
    if filter.match_rule::<NoCompareNegZero>() {
        rules.push::<NoCompareNegZero>();
    }
    if filter.match_rule::<NoDebugger>() {
        rules.push::<NoDebugger>();
    }
    if filter.match_rule::<NoDelete>() {
        rules.push::<NoDelete>();
    }
    if filter.match_rule::<NoDoubleEquals>() {
        rules.push::<NoDoubleEquals>();
    }
    if filter.match_rule::<NoEmptyPattern>() {
        rules.push::<NoEmptyPattern>();
    }
    if filter.match_rule::<NoImplicitBoolean>() {
        rules.push::<NoImplicitBoolean>();
    }
    if filter.match_rule::<NoMultipleSpacesInRegularExpressionLiterals>() {
        rules.push::<NoMultipleSpacesInRegularExpressionLiterals>();
    }
    if filter.match_rule::<NoNegationElse>() {
        rules.push::<NoNegationElse>();
    }
    if filter.match_rule::<NoSparseArray>() {
        rules.push::<NoSparseArray>();
    }
    if filter.match_rule::<NoUnnecessaryContinue>() {
        rules.push::<NoUnnecessaryContinue>();
    }
    if filter.match_rule::<NoUnsafeNegation>() {
        rules.push::<NoUnsafeNegation>();
    }
    if filter.match_rule::<NoUnusedTemplateLiteral>() {
        rules.push::<NoUnusedTemplateLiteral>();
    }
    if filter.match_rule::<UseBlockStatements>() {
        rules.push::<UseBlockStatements>();
    }
    if filter.match_rule::<UseSelfClosingElements>() {
        rules.push::<UseSelfClosingElements>();
    }
    if filter.match_rule::<UseShorthandArrayType>() {
        rules.push::<UseShorthandArrayType>();
    }
    if filter.match_rule::<UseSimplifiedLogicExpression>() {
        rules.push::<UseSimplifiedLogicExpression>();
    }
    if filter.match_rule::<UseSingleCaseStatement>() {
        rules.push::<UseSingleCaseStatement>();
    }
    if filter.match_rule::<UseSingleVarDeclarator>() {
        rules.push::<UseSingleVarDeclarator>();
    }
    if filter.match_rule::<UseValidTypeof>() {
        rules.push::<UseValidTypeof>();
    }
    if filter.match_rule::<UseWhile>() {
        rules.push::<UseWhile>();
    }
    if filter.match_rule::<FlipBinExp>() {
        rules.push::<FlipBinExp>();
    }
    rules
}
