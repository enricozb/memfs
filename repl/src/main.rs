mod util;

use std::{io::Write, path::PathBuf};

use clap::Parser;
use fs::Filesystem;
use session::{Result, Session};

#[derive(Parser)]
#[command(
  override_usage = "A in-memory filesystem repl. Use ctrl-c to exit.",
  no_binary_name = true,
  disable_help_flag = true
)]
enum Command {
  /// Change directory.
  Cd { path: PathBuf },

  /// List directory entries.
  Ls {
    #[clap(default_value = ".")]
    path: PathBuf,
  },

  /// Create a new directory.
  Mkdir { path: PathBuf },

  /// Creates an empty file.
  Touch { path: PathBuf },

  /// Fills a file with random data.
  Fill { path: PathBuf },

  /// Prints a file's content.
  Cat { path: PathBuf },

  /// Remove a directory or file.
  Rm { path: PathBuf },

  /// Move a file or directory. The destination will be the source's new name,
  /// as opposed to the source's new parent. This will overwrite the destination if one exists.
  Mv { src: PathBuf, dst: PathBuf },

  /// List contents of directories in a tree-like format.
  Tree {
    #[clap(default_value = ".")]
    path: PathBuf,
  },
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
      Command::Mkdir { path } => self.session.create_directory(path)?,
      Command::Touch { path } => self.session.create_file(path)?,
      Command::Fill { path } => self.session.write_file(path, crate::util::random_ascii(100))?,
      Command::Cat { path } => println!("{}", self.session.read_file(path)?),
      Command::Rm { path } => self.session.remove(path)?,
      Command::Mv { src, dst } => self.session.move_entry(src, dst)?,

      Command::Tree { path } => {
        let root_depth = path.components().count();

        self.session.walk(path, |path, entry| {
          let depth = "  ".repeat(path.components().count() - root_depth);

          println!("{depth}· {name:?}", name = entry.name());
        })?;
      }

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
