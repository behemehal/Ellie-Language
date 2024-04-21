use super::TypeTranspiler;
use ellie_core::definite::types::function::Function;

impl TypeTranspiler for Function {
    fn transpile(&self, _options: &mut super::TypeTranspilerOptions) {
        todo!()
    }
}
