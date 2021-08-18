#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

struct App<'a> {
    data: Vec<(&'a str, u64)>,
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            data: (0..100).map(|x| ("", x)).collect::<Vec<_>>(),
            items: StatefulList::with_items(vec![("Bouble Sort", 1), ("Insertion Sort", 2)]),
        }
    }

    fn update(&mut self) {}
}

struct SortGenerator { }

impl

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            let list = List::new(
                app.items
                    .items
                    .iter()
                    .map(|x| ListItem::new(x.0))
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");
            f.render_stateful_widget(list, chunks[0], &mut app.items.state);

            let barchart = BarChart::default()
                .block(Block::default().borders(Borders::ALL))
                .data(&app.data)
                .bar_width(chunks[1].width / 100)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Green))
                .value_style(
                    Style::default()
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_widget(barchart, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Up => {
                    app.items.previous();
                }
                Key::Char(' ') => {
                  
                },
                _ => {}
            },
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
