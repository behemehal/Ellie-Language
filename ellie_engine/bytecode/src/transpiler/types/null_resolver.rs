use super::TypeTranspiler;
use ellie_core::definite::types::null_resolver::NullResolver;

impl TypeTranspiler for NullResolver {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        self.target.transpile(options)
    }
}
