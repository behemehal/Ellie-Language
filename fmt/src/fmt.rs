use crate::renderer::CodeRenderer;
use crate::renderer::State;
use ellie_core::utils::PageExport;
pub use ellie_tokenizer;
use ellie_tokenizer::tokenizer::Page;

#[derive(Clone, Debug, Copy)]
pub struct FormatterOptions {
    pub float_starts_with_dot: bool,
    pub extend_array: bool,
    pub leave_space_after_comma: bool,
    pub use_shorts: bool,
    pub space_before_type_colon: bool,
    pub space_between_operators: bool,
    pub render_brace_next_line: bool,
    pub is_cr_lf: bool,
    pub tab_size: usize,
    pub use_tabs: bool,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            float_starts_with_dot: false,
            extend_array: false,
            leave_space_after_comma: true,
            use_shorts: true,
            space_before_type_colon: false,
            render_brace_next_line: true,
            space_between_operators: true,
            is_cr_lf: false,
            tab_size: 4,
            use_tabs: true,
        }
    }
}

impl FormatterOptions {
    pub fn render_line_ending(&self) -> String {
        if self.is_cr_lf {
            String::from("\r\n")
        } else {
            String::from("\n")
        }
    }
}

pub struct Formatter {
    pub options: FormatterOptions,
    pub export: PageExport<Page>,
}

pub struct FormattedPage {
    pub path: String,
    pub content: String,
}

impl Formatter {
    pub fn new(options: FormatterOptions, export: PageExport<Page>) -> Formatter {
        Formatter { options, export }
    }

    fn format_page(&self, page: &Page) -> String {
        let mut output = String::new();

        for item in page.items.iter() {
            output += &item.render(&State::empty_state(), &self.options);
        }
        output
    }

    pub fn format(&self) -> Vec<FormattedPage> {
        let mut formatted_pages = Vec::new();
        for page in self.export.pages.iter() {
            let content = self.format_page(page);
            formatted_pages.push(FormattedPage {
                path: page.path.clone(),
                content,
            });
        }
        formatted_pages
    }
}
