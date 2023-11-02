use std::{
  path::Path,
  env,
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
    if i.to_lowercase().strip_suffix(".gitignore").unwrap_or(&i.to_lowercase()).contains(&p.to_lowercase()) {
      p3.push(i.to_string());
    }
  };
  let mut _completions = completions!();
  _completions.extend(p3);
  _completions
}

fn comment_delete(a: String, b: bool) -> String {
  let _iter = a.clone();
  let mut list: Vec<String> = Vec::new();
  if b {
    for i in _iter.lines() {
      if i != "" && !i.starts_with('#') {
        list.push(i.to_string());
      }
    }
  } else {
    return a;
  }
  list.join("\n")
}

fn main() {
  let mut selections = Vec::<String>::new();
  let _binding = Path::new(&home_dir().unwrap()).join(".ignofierplus");
  let pathforgit = _binding.to_str().unwrap();
  println!("{}", pathforgit);

  if Path::new(&home_dir().unwrap()).join(".ignofierplus").is_dir() {
    let _original = env::current_dir().unwrap();
    env::set_current_dir(_binding.clone()).unwrap();
    Command::new("git").args(["pull"]).spawn().unwrap().wait().unwrap();
    env::set_current_dir(_original).unwrap();
  } else {
    Command::new("git").args(["clone", "https://github.com/github/gitignore", pathforgit]).spawn().unwrap().wait().unwrap();
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

  let selection = requestty::prompt_one(ques).expect("no selected string").try_into_string().expect("can't make select to string");

  if !Path::new(".gitignore").is_file() {
    let mut file = File::create(".gitignore").unwrap();
    file.write_all(format!("\n# {} by ignofierplus\n{}", selection, read_to_string(format!("{pathforgit}/{selection}")).unwrap()).as_bytes()).unwrap();
  } else {
    let input = Question::select("theme")
    .message(".gitignore exists, append or overwrite?")
    .choices(vec!["append", "overwrite", "cancel"])
    .build();
    let _selection = requestty::prompt_one(input).unwrap().try_into_list_item().unwrap().text;
    if _selection == "cancel" {
      println!("Canceled");
      return;
    }
    let _append: bool = _selection == "append";
    let mut file = OpenOptions::new()
      .write(true)
      .append(_append)
      .open(".gitignore")
      .unwrap();

    if !_append {
      file.set_len(0).unwrap();
    }

    let input: Question<'_> = Question::select("b")
      .message("Do you want a delete comments in .gitignore? (y/n)")
      .choices(vec!["y", "n"])
      .build();

    let _selection: bool;

    if requestty::prompt_one(input).unwrap().try_into_list_item().unwrap().text == "y" {
      _selection = true;
    } else {
      _selection = false;
    }

    let _path = read_to_string(format!("{pathforgit}/{selection}")).unwrap();
    let strings = _path.trim_end_matches('\n');
    
    if let Err(e) = writeln!(file, "\n# {} by ignofierplus\n{}",
      selection,
      comment_delete(strings.to_string(), _selection)
    ) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }

  println!("Your .gitignore ready!")
}
