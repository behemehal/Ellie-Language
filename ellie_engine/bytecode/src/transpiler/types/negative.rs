use super::TypeTranspiler;
use ellie_core::definite::types::negative::Negative;

impl TypeTranspiler for Negative {
    fn transpile(&self, _options: &mut super::TypeTranspilerOptions) {
        todo!()
    }
}
