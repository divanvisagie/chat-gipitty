use crate::{
    chatgpt::{GptClient, Role},
    utils::get_logged_in_user_name,
};
use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

pub fn run_tui_mode(cli: &mut GptClient) -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, cli)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    cli: &mut GptClient,
) -> Result<(), Box<dyn Error>> {
    let username = get_logged_in_user_name();
    let mut input_buffer = String::new();

    Ok(loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chat_area = Rect::new(0, 0, size.width - 7, size.height - 1); // The chat takes up everything except the last line
            let input_area = Rect::new(0, size.height - 1, size.width, 1); // The input takes up only the last line

            let chat_text: Vec<String> = cli
                .messages
                .iter()
                .map(|msg| match msg.role.parse().expect("can't parse role") {
                    Role::User => format!("{}> {}", username, msg.content),
                    _ => format!("{}> {}", msg.role, msg.content),
                })
                .collect();

            let chat_paragraph = Paragraph::new(chat_text.join("\n")).wrap(Wrap { trim: true });
            frame.render_widget(chat_paragraph, chat_area);

            let input_paragraph = Paragraph::new(format!("{}> {}", username, input_buffer));
            frame.render_widget(input_paragraph, input_area);
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => input_buffer.push(c),
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Enter => {
                        cli.add_message(Role::User, input_buffer.clone());
                        cli.complete();
                        input_buffer.clear();
                    }
                    _ => {
                        println!("key_event: {:?}", key_event);
                    }
                }
            }
        }
    })
}
