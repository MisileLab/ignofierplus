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

/// Auto complete input of gitignore
fn auto_complete(input: String, filelist: &Vec<String>) -> Completions<String> {
  let mut completions = Vec::<String>::new();
  completions.push(input.clone());
  for i in filelist {
    if i.to_lowercase().strip_suffix(".gitignore").unwrap_or(&i.to_lowercase()).contains(&input.to_lowercase()) {
      completions.push(i.to_string());
    }
  };
  let mut _completions = completions!();
  _completions.extend(completions);
  _completions
}

/// delete gitignore's comment
fn delete_comments(string: String) -> String {
  let mut list: Vec<String> = Vec::new();
  for i in string.lines() {
    if i != "" && !i.starts_with('#') {
      list.push(i.to_string());
    }
  }
  list.join("\n")
}

fn main() {
  let mut selections = Vec::<String>::new();
  let homedir = Path::new(&home_dir().expect("can't find home path")).join(".ignofierplus");
  let homedir_str = homedir.to_str().unwrap();
  println!("{}", homedir_str);

  // I can't how to handling it is git version or not
  if homedir.is_dir() {
    let _original = env::current_dir().unwrap();
    env::set_current_dir(homedir.clone()).unwrap();
    Command::new("git").args(["pull"]).spawn().unwrap().wait().unwrap();
    env::set_current_dir(_original).unwrap();
  } else {
    Command::new("git").args(["clone", "https://github.com/github/gitignore", homedir_str]).spawn().unwrap().wait().unwrap();
  }

  for entry in WalkDir::new(&homedir) {
    let entry = entry.unwrap();
    if entry.path().to_str().unwrap().to_string().ends_with(".gitignore") {
      selections.push(entry.path().to_str().unwrap().to_string().trim_start_matches(homedir_str).trim_start_matches('/').trim_start_matches('\\').to_string());
    }
  }

  let ques = requestty::Question::input("a")
    .message("Choose .gitignore template")
    .auto_complete(|p, _| auto_complete(p, &selections))
    .validate(|p, _| {
      if homedir.join(p).exists() {
          Ok(())
      } else {
          Err(format!("file `{}` doesn't exist", p))
      }
  })
  .build();

  let selection = requestty::prompt_one(ques).expect("no selected string").try_into_string().expect("can't make select to string");
  let mut _append = true;
  if Path::new(".gitignore").is_file() {
    let input = Question::select("gitignorexist")
    .message(".gitignore exists, append or overwrite?")
    .choices(vec!["append", "overwrite", "cancel"])
    .build();
    let _selection = requestty::prompt_one(input).unwrap().try_into_list_item().unwrap().text;
    if _selection == "cancel" {
      println!("Canceled");
      return;
    }
    _append = _selection == "append";
  } else {
    File::create(".gitignore").unwrap();
  }
  let mut file = OpenOptions::new()
    .write(true)
    .append(_append)
    .open(".gitignore")
    .unwrap();

  if !_append {
    file.set_len(0).unwrap();
  }

  let input: Question<'_> = Question::select("b")
    .message("Do you want to delete comments in .gitignore? (y/n)")
    .choices(vec!["y", "n"])
    .build();

  let delete_comment: bool;

  if requestty::prompt_one(input).unwrap().try_into_list_item().unwrap().text == "y" {
    delete_comment = true;
  } else {
    delete_comment = false;
  }

  let _path = read_to_string(format!("{homedir_str}/{selection}")).unwrap();
  let strings = _path.trim_end_matches('\n').to_string();
  
  if let Err(e) = writeln!(file, "\n# {} by ignofierplus\n{}",
    selection,
    if delete_comment { delete_comments(strings) } else { strings }
  ) {
    eprintln!("Couldn't write to file: {}", e);
  }

  println!("Your .gitignore ready!")
}
