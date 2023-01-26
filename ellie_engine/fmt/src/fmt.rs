use crate::renderer::CodeRenderer;
use crate::renderer::State;
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
            use_tabs: false,
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
}

pub struct FormattedPage {
    pub path: String,
    pub content: String,
}

struct FormatedFile {
    pub lines: Vec<String>,
    pub line_ending: String,
}

impl FormatedFile {
    pub fn new(line_ending: String) -> FormatedFile {
        FormatedFile {
            lines: vec![],
            line_ending,
        }
    }

    fn insert_element_to_line(&mut self, line: usize, element: String) {
        if self.lines.len() < line {
            self.lines.resize(line, self.line_ending.clone());
        }
        let lines = element
            .split(&self.line_ending)
            .map(str::to_string)
            .filter(|x| x != &"")
            .collect::<Vec<String>>();
        self.lines.extend(lines);
    }

    fn render_out(&self) -> String {
        self.lines.join(&self.line_ending)
    }
}

impl Formatter {
    pub fn new(options: FormatterOptions) -> Formatter {
        Formatter { options }
    }

    pub fn format_page(&self, page: &Page) -> String {
        let mut output = FormatedFile::new(self.options.render_line_ending());

        for item in page.items.iter() {
            let item_pos = item.get_pos();

            let formated_item = item.render(&State::empty_state(), &self.options);
            output.insert_element_to_line(item_pos.range_start.0, formated_item);
        }
        output.render_out()
    }
}
