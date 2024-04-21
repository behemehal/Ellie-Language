use super::TypeTranspiler;
use ellie_core::definite::types::enum_data::EnumData;

impl TypeTranspiler for EnumData {
    fn transpile(&self, _options: &mut super::TypeTranspilerOptions) {
        todo!()
    }
}
