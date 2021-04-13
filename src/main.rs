#![allow(dead_code)]

mod app;
mod config;
mod controller;
mod core;
mod keys;
mod style;
mod ui;
mod utils;

use anyhow::{bail, Result};
use app::App;
use crossbeam_channel::{tick, Receiver, Select};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{
    borrow::Borrow,
    io::{self, Write},
    rc::Rc,
    time::{Duration, Instant},
};
use style::Theme;

use simplelog::{Config, LevelFilter, TermLogger};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::UI;

fn main() -> Result<()> {
    setup_logging()?;
    setup_terminal()?;

    let mut terminal = start_terminal(io::stdout())?;
    let mut app = App::new(true);
    let mut ui = UI::new();

    app.start()?;

    loop {
        // Simultaion
        app.step()?;

        // Terminal
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(ev) => {
                    controller::key_event(&mut app, ev, &mut ui)?;
                }
                Event::Mouse(ev) => {}
                Event::Resize(width, height) => {}
            }
        }
        draw(&mut terminal, &app, &ui)?;
        if app.should_quit {
            shutdown_terminal()?;
            break;
        }
    }

    Ok(())
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App, ui: &UI) -> Result<()> {
    terminal.draw(|mut f| {
        if let Err(e) = ui.draw(&mut f, app) {
            log::error!("failed drawing");
        }
    })?;

    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn start_terminal<W: Write>(buf: W) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

fn shutdown_terminal() -> Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn setup_logging() -> Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        simplelog::TerminalMode::Mixed,
    );

    Ok(())
}
