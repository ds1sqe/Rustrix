use std::borrow::Cow;

use super::AbstractDomain;

pub trait AbstractEnviroment<Value, D: AbstractDomain + Clone> {
    fn get(&self, variable: &Value) -> Cow<D>;
    fn set(&mut self, variable: Value, domain: D);
}
