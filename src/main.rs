use std::{
  path::Path,
  process::Command,
  fs::{File, read_to_string, OpenOptions},
  io::Write
};
use dirs::home_dir;
use walkdir::WalkDir;
use requestty::{question::{completions, Completions}, Question};

fn auto_complete(p: String, p2: &Vec<String>) -> Completions<String> {
  let mut p3 = Vec::<String>::new();
  p3.push(p.clone());
  for i in p2 {
    if i.starts_with(&p) {
      p3.push(i.to_string());
    }
  };
  let mut _completions = completions!();
  _completions.extend(p3);
  _completions
}

fn main() {
  let mut selections = Vec::<String>::new();
  let _binding = Path::new(&home_dir().unwrap()).join(".ignofierplus");
  let pathforgit = _binding.to_str().unwrap();

  if Path::new(&home_dir().unwrap()).join(".ignofierplus").is_dir() {
    Command::new("git").args(["pull", pathforgit]);
  } else {
    Command::new("git").args(["clone", "https://github.com/github/gitignore", pathforgit]);
  }

  for entry in WalkDir::new(Path::new(&home_dir().unwrap()).join(".ignofierplus")) {
    let entry = entry.unwrap();
    if entry.path().to_str().unwrap().to_string().ends_with(".gitignore") {
      selections.push(entry.path().to_str().unwrap().to_string().trim_start_matches(pathforgit).trim_start_matches('/').trim_start_matches('\\').to_string());
    }
  }

  let ques = requestty::Question::input("a")
    .message("Choose .gitignore template")
    .auto_complete(|p, _| auto_complete(p, &selections))
    .validate(|p, _| {
      if _binding.join(p).exists() {
          Ok(())
      } else {
          Err(format!("file `{}` doesn't exist", p))
      }
  })
  .build();

  let selection = requestty::prompt_one(ques).unwrap().try_into_string().unwrap();

  if !Path::new(".gitignore").is_file() {
    let mut file = File::create(".gitignore").unwrap();
    file.write_all(format!("# {} by ignofierplus\n{}", selection, read_to_string(format!("{pathforgit}/{selection}")).unwrap()).as_bytes()).unwrap();
  } else {
    let input = Question::select("theme")
    .message(".gitignore exists, append or overwrite?")
    .choices(vec!["append", "overwrite"])
    .build();
    let _selection = requestty::prompt_one(input).unwrap().try_into_list_item().unwrap().text;
    let _append: bool = _selection == "append";
    let mut file = OpenOptions::new()
      .write(true)
      .append(_append)
      .open(".gitignore")
      .unwrap();

    if !_append {
      file.set_len(0).unwrap();
    }
    
    if let Err(e) = writeln!(file, "# {} by ignofierplus\n{}",
      selection,
      read_to_string(format!("{pathforgit}/{selection}")).unwrap().trim_end_matches('\n')
    ) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }

  println!("Your .gitignore ready!")
}