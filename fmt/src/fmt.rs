pub struct Formatter {
    pub target_page: Page,

    current_position: Cursor,
}

impl Formatter {
    pub fn new(page: Page) -> Formatter {
        Formatter { target_page: page }
    }

    fn format_item(&mut self, line: &str) {
        self.current_position.move_to_next_line();
        self.current_position.write_line(line);
    }

    pub fn format(&self) -> String {

        for item in self.target_page.items.iter() {
            self.format_item(item);
        }

    }
}
