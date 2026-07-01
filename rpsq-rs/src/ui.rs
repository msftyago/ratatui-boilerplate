use crate::app::App;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![" Quit ".into(), "<q> ".red().bold()]);
        let block = Block::bordered()
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let sample_text = Line::from("Gartensia CLI");

        Paragraph::new(sample_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
