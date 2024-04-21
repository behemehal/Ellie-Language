use super::TypeTranspiler;

pub struct Dynamic;

impl TypeTranspiler for Dynamic {
    fn transpile(&self, _options: &mut super::TypeTranspilerOptions) {
        todo!()
    }
}
