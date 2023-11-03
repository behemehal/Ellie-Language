use crate::renderer::CodeRenderer;
use crate::renderer::State;
pub use ellie_tokenizer;
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::tokenizer::Page;

#[derive(Clone, Debug, Copy)]
pub struct FormatterOptions {
    pub decimal_starts_with_dot: bool,
    pub decorate_float_with_f_tag: bool,
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
            decimal_starts_with_dot: true,
            decorate_float_with_f_tag: true,
            extend_array: false,
            leave_space_after_comma: true,
            use_shorts: true,
            space_before_type_colon: true,
            render_brace_next_line: false,
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

    pub fn render_tab(&self) -> String {
        if self.use_tabs {
            String::from("\t")
        } else {
            String::from("    ")
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

#[derive(Debug)]
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
            self.lines.resize(line, String::new());
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
        let mut last_element_is_fn_or_class = false;
        for item in &page.items {
            match item {
                Processors::Function(_) | Processors::Class(_) => {
                    output.lines.push(String::new());
                    last_element_is_fn_or_class = true;
                }
                Processors::Comment(e) => {
                    if e.line_comment && e.pos.range_start.0 == output.lines.len() - 1 {
                        let formated_item = item.render(&State::empty_state(), &self.options);
                        let last_line = output.lines.last_mut().unwrap();
                        last_line.push(' ');
                        last_line.push_str(&formated_item);
                        continue;
                    }
                }
                _ => {
                    if last_element_is_fn_or_class {
                        output.lines.push(String::new());
                        last_element_is_fn_or_class = false;
                    }
                }
            };
            let formated_item = item.render(&State::empty_state(), &self.options);
            output.insert_element_to_line(item.get_pos().range_start.0, formated_item);
        }
        output.render_out()
    }
}
