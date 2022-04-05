use ellie_tokenizer::syntax::items::getter_call::GetterCall;

impl super::Processor for GetterCall {
    fn process(self, parser: &mut super::Parser, page_id: u64) -> bool {
        match super::type_processor::process(self.data, parser, page_id, None) {
            Ok(_) => true,
            Err(e) => {
                parser.informations.extend(&e);
                false
            },
        }
    }
}
