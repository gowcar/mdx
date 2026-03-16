pub struct Viewport {
    pub offset: u16,
    pub height: u16,
    pub content_height: u16,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            offset: 0,
            height: 0,
            content_height: 0,
        }
    }

    pub fn scroll_down(&mut self, lines: u16) {
        let max = self.content_height.saturating_sub(self.height);
        self.offset = (self.offset + lines).min(max);
    }

    pub fn scroll_up(&mut self, lines: u16) {
        self.offset = self.offset.saturating_sub(lines);
    }

    pub fn page_down(&mut self) {
        self.scroll_down(self.height / 2);
    }

    pub fn page_up(&mut self) {
        self.scroll_up(self.height / 2);
    }

    pub fn full_page_down(&mut self) {
        self.scroll_down(self.height.saturating_sub(2));
    }

    pub fn full_page_up(&mut self) {
        self.scroll_up(self.height.saturating_sub(2));
    }

    pub fn go_top(&mut self) {
        self.offset = 0;
    }

    pub fn go_bottom(&mut self) {
        self.offset = self.content_height.saturating_sub(self.height);
    }

    pub fn percentage(&self) -> u16 {
        if self.content_height == 0 {
            return 100;
        }
        let max = self.content_height.saturating_sub(self.height);
        if max == 0 {
            return 100;
        }
        ((self.offset as u32 * 100) / max as u32).min(100) as u16
    }

    pub fn current_line(&self) -> u16 {
        self.offset + 1
    }
}
