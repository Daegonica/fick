use std::io;

use crossterm::event::{KeyEventKind, KeyCode};

use ratatui::{
    layout::{Layout, Constraint, Rect},
    style::{Style, Color, Stylize, Modifier},
    symbols::border,
    text::{Line, Span},
    widgets::{Widget, Block, Gauge},
    DefaultTerminal, Frame,
};


pub fn draw() -> io::Result<()> {

    let mut terminal = ratatui::init();
    let mut app = App { exit: false, progress_bar_color: Color::Green };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
    progress_bar_color: Color,
}

impl App {

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Esc {
            self.exit = true;
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            if self.progress_bar_color == Color::Green {
                self.progress_bar_color = Color::Yellow;
            } else {
                self.progress_bar_color = Color::Green;
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized 
    {
        let vertical_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from(
            Span::styled (
                "Daegonica Software",
                Style::default().add_modifier(Modifier::BOLD)
            )
        ).render(title_area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Esc>".blue().bold()
        ]).centered();

        let block = Block::bordered()
            .title(Line::from("Background processes "))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!("Process 1: 50%"))
            .ratio(0.5);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },buf);
    }
}