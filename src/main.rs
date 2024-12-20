use std::error::Error;
use rusty_audio::Audio;
use std::io;
use std::time::Duration;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};


fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "src/explode.wav");
    audio.add("lose", "src/lose.wav");
    audio.add("move", "src/move.wav");
    audio.add("pew", "src/pew.wav");
    audio.add("startup", "src/startup.wav");
    audio.add("win","src/win.wav");
    audio.play("startup");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;                    
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
