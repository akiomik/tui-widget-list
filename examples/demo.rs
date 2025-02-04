use std::error::Error;
use std::io::{stdout, Stdout};

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{Frame, Terminal};
use tui_widget_list::{List, ListState, Listable};

#[derive(Debug, Clone)]
pub struct MyListItem {
    title: String,
    content: Vec<String>,
    style: Style,
    height: usize,
    expand: bool,
}

impl MyListItem {
    pub fn new(title: &str, content: Vec<String>) -> Self {
        Self {
            title: title.to_string(),
            content,
            style: Style::default(),
            height: 2,
            expand: false,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn expand(mut self) -> Self {
        self.expand = true;
        self.height = 3 + self.content.len();
        self
    }
}

impl Listable for MyListItem {
    fn height(&self) -> usize {
        self.height
    }

    fn highlight(self) -> Self {
        self.style(THEME.selection).expand()
    }
}

impl Widget for MyListItem {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![Line::styled(self.title, self.style)];
        if self.expand {
            lines.push(Line::from(String::new()));
            lines.extend(self.content.into_iter().map(|x| Line::from(x)));
            lines.push(Line::from(String::new()));
        }
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(self.style)
            .render(area, buf);
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;

    let app = App::new();
    run(&mut terminal, app).unwrap();

    reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    panic_hook();

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

/// Shutdown gracefully
fn panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}

pub struct App {
    pub state: ListState,
}

impl App {
    pub fn new() -> App {
        let state = ListState::default();
        App { state }
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up | KeyCode::Char('k') => app.state.previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.state.next(),
                    _ => {}
                }
            }
        }
    }
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let todos1: Vec<String> = vec![
        String::from("1. Exercise for 30 minutes"),
        String::from("2. Have a breakfast"),
        String::from("3. Work on the project for 2 hours"),
        String::from("4. Read a book for 1 hour"),
        String::from("5. Cook dinner"),
    ];
    let todos2: Vec<String> = vec![
        String::from("1. Attend a team meeting at 10 AM"),
        String::from("2. Reply to emails"),
        String::from("3. Prepare lunch"),
        String::from("4. Go running for 1 hour"),
    ];
    let list = List::new(vec![
        MyListItem::new("Monday", todos1.clone()),
        MyListItem::new("Tuesday", todos2.clone()),
        MyListItem::new("Wednesday", todos1.clone()),
        MyListItem::new("Thursday", todos2.clone()),
        MyListItem::new("Friday", todos1.clone()),
        MyListItem::new("Saturday", todos2),
        MyListItem::new("Sunday", todos1),
    ])
    .style(THEME.root);
    f.render_stateful_widget(list, f.size(), &mut app.state);
}

pub struct Theme {
    pub root: Style,
    pub content: Style,
    pub selection: Style,
}

pub const THEME: Theme = Theme {
    root: Style::new().bg(DARK_BLUE),
    content: Style::new().bg(DARK_BLUE).fg(LIGHT_GRAY),
    selection: Style::new().bg(DARK_PURPLE).fg(LIGHT_GRAY),
};

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
const DARK_PURPLE: Color = Color::Indexed(55);
const LIGHT_GRAY: Color = Color::Indexed(250);
