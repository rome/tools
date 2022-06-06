use crate::JsAnyArrowFunctionParameters;
use rome_rowan::AstSeparatedList;

impl JsAnyArrowFunctionParameters {
    pub fn len(&self) -> usize {
        match self {
            JsAnyArrowFunctionParameters::JsAnyBinding(_) => 1,
            JsAnyArrowFunctionParameters::JsParameters(parameters) => parameters.items().len(),
        }
    }
}
