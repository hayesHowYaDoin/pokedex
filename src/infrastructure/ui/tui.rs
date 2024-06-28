use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
  cursor,
  event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, EventStream},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend as Backend;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use futures::{FutureExt, StreamExt};

pub type TuiBackend = ratatui::backend::CrosstermBackend<std::io::Stderr>;
pub type Terminal = ratatui::Terminal<Backend<std::io::Stderr>>;

const TICK_RATE: Duration = Duration::from_millis(100);

pub enum Event {
    AppTick,
    Key(KeyEvent),
    Error,
}

pub struct Tui {
    pub terminal: Terminal,
    task: JoinHandle<()>,
    cancellation_token: CancellationToken,
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
        let task = tokio::spawn(async {});
        let cancellation_token = CancellationToken::new();
        let (tx, rx) = mpsc::unbounded_channel();
    
        Ok(Tui{terminal, task, cancellation_token, tx, rx})
    }

    pub fn start(&mut self) {
        self.cancel();
        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();
        let _tx = self.tx.clone();

        self.task = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut interval = tokio::time::interval(TICK_RATE);
            
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        break;
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(CrosstermEvent::Key(key))) => {
                                if key.kind == KeyEventKind::Press {
                                    _tx.send(Event::Key(key)).unwrap();
                                }
                            }
                            Some(Ok(_)) => {}, // unimplemented!(),
                            Some(Err(_)) => {
                                _tx.send(Event::Error).unwrap();
                            }
                            None => {},
                        }
                    }          
                    _ = delay => {
                        _tx.send(Event::AppTick).unwrap();
                    },
                }
            }
        });
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(())
    }
    
    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
          self.flush()?;
          crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
          crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
          std::thread::sleep(Duration::from_millis(1));
          counter += 1;
          if counter > 50 {
            self.task.abort();
          }
          if counter > 100 {
            log::error!("Failed to abort task in 100 milliseconds for unknown reason");
            break;
          }
        }
        Ok(())
      }
    
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }
    
    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
        Ok(())
    }
    
    pub fn resume(&mut self) -> Result<()> {
        self.enter()?;
        Ok(())
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;
  
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}
  
impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
  
impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}