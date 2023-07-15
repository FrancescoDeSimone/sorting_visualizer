mod array_state_event;
mod sorting_algorithms;
mod ui;
mod util;

use crate::util::event::{Event, Events};

use crate::ui::App;

use sorting_algorithms::SortingAlgorithm;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders, List, ListItem};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new(vec![
        SortingAlgorithm::BubbleSort,
        SortingAlgorithm::InsertionSort,
        SortingAlgorithm::SelectionSort,
    ]);
    app.sorting_algoritms.state.select(Some(0));
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let list = List::new(
                app.sorting_algoritms
                    .items
                    .iter()
                    .map(|x| ListItem::new(x.get_name()))
                    .collect::<Vec<_>>(),
            )
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");
            f.render_stateful_widget(list, chunks[0], &mut app.sorting_algoritms.state);

            let data = app.get_data();
            let barchart = BarChart::default()
                .block(Block::default().borders(Borders::ALL))
                .data(&data)
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
                Key::Down => {
                    app.sorting_algoritms.next();
                    app.reset();
                }
                Key::Left => app.go_back(),
                Key::Right => app.go_forward(),
                Key::Up => {
                    app.sorting_algoritms.previous();
                    app.reset();
                }
                Key::Char(' ') => app.run_sort(),
                Key::Char('r') => app.reset(),
                _ => {}
            },
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
