use std::{io::Write, path::PathBuf};

use clap::Parser;
use fs::Filesystem;
use session::{Result, Session};

#[derive(Parser)]
#[command(override_usage = "<COMMAND> [ARGS and OPTIONS]", no_binary_name = true, disable_help_flag = true)]
enum Command {
  /// Change directory.
  Cd { path: PathBuf },

  /// List directory entries.
  Ls {
    #[clap(default_value = ".")]
    path: PathBuf,
  },

  /// Create a new directory.
  Mkdir { name: String },

  /// Remove a directory or file.
  Rm { path: PathBuf },
}

struct Repl {
  session: Session,
}

impl Repl {
  pub fn new(filesystem: Filesystem) -> Self {
    Self {
      session: Session::new(filesystem),
    }
  }

  fn get_line(&self) -> String {
    // TODO: don't unwrap
    print!("{current_directory:?} >>> ", current_directory = self.session.current_directory());
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line
  }

  fn handle_command(&mut self, command: Command) -> Result<()> {
    match command {
      Command::Cd { path } => self.session.change_directory(path)?,
      Command::Mkdir { name } => self.session.create_directory(name)?,
      Command::Rm { path } => self.session.remove(path)?,
      Command::Ls { path } => {
        for entry in self.session.list_directory(path)? {
          println!(
            "{kind} {name:?}",
            kind = if entry.is_directory() { "d" } else { "f" },
            name = entry.name()
          );
        }
      }
    }

    Ok(())
  }
}

fn main() {
  let mut repl = Repl::new(Filesystem::new());

  loop {
    let line = repl.get_line();

    let command = match Command::try_parse_from(line.trim().split(' ')) {
      Ok(command) => command,
      Err(err) => {
        println!("{err}");
        continue;
      }
    };

    if let Err(err) = repl.handle_command(command) {
      println!("{err}");
    }
  }
}
