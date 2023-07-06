mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};

use futures_core::Stream;
use futures_util::pin_mut;
use futures_util::StreamExt;
use sorting_generator;
use std::pin::Pin;
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
    items: StatefulList<(&'a str, usize, Pin<Box<dyn Stream<Item = Vec<(&'a str,u64)>>>>)>,
    pause: bool,
}

impl<'a> App<'a> {
    fn new() -> App<'static> {
        App {
            data: Vec::new(),
            items: StatefulList::with_items(vec![
                ("Bubble Sort", 0, Box::pin(sorting_generator::bubble_sort())),
                ("Insertion Sort", 1, Box::pin(sorting_generator::insertion_sort()),),
                ("Selection Sort", 2, Box::pin(sorting_generator::selection_sort()),),
            ]),
            pause: true,
        }
    }

    async fn update(&mut self) {
        match self.items.state.selected() {
            Some(_) => {
                if !self.pause {
                    let s = &mut self.items.items[self.items.state.selected().unwrap()].2;
                    pin_mut!(s);
                    if let Some(value) = s.next().await {
                        self.data = value; //.iter().map(|x| ("", *x)).collect();
                    }
                }
            }
            None => {
                self.items.state.select(Some(0));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(20)].as_ref())
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
                .bar_width(chunks[1].width / 55)
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
                Key::Down => {
                    app.items.next();
                    app.pause = true;
                }
                Key::Up => {
                    app.items.previous();
                    app.pause = true;
                }
                Key::Char(' ') => {
                    app.pause = !app.pause;
                }
                _ => {}
            },
            Event::Tick => {
                app.update().await;
            }
        }
    }

    Ok(())
}
