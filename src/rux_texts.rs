use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::rux_debug::dbg_erro;
use crate::rux_debug::{dbg_call, dbg_reav, dbg_tell};
use crate::{RubxError, RubxResult};

pub static LINE_SPACE_CHARS: &[char] = &[' ', '\t'];
pub static LINE_BREAK_CHARS: &[char] = &['\n', '\r'];
pub static BRACKETS_CHARS: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];
pub static QUOTATION_CHARS: &[char] = &['\'', '"'];

pub fn ask(message: &str) -> Result<String, RubxError> {
  dbg_call!(message);
  print!("{} ", message);
  std::io::stdout().flush().map_err(|err| dbg_erro!(err))?;
  let mut buffer = String::new();
  std::io::stdin()
    .read_line(&mut buffer)
    .map_err(|err| dbg_erro!(err))?;
  Ok(buffer.trim().to_string())
}

pub fn ask_int(message: &str) -> Result<i32, RubxError> {
  dbg_call!(message);
  print!("{} ", message);
  std::io::stdout().flush().map_err(|err| dbg_erro!(err))?;
  let mut buffer = String::new();
  std::io::stdin()
    .read_line(&mut buffer)
    .map_err(|err| dbg_erro!(err))?;
  let result = buffer.trim().parse::<i32>().map_err(|err| dbg_erro!(err))?;
  Ok(result)
}

pub fn ask_float(message: &str) -> Result<f64, RubxError> {
  dbg_call!(message);
  print!("{} ", message);
  std::io::stdout().flush().map_err(|err| dbg_erro!(err))?;
  let mut buffer = String::new();
  std::io::stdin()
    .read_line(&mut buffer)
    .map_err(|err| dbg_erro!(err))?;
  let result = buffer.trim().parse::<f64>().map_err(|err| dbg_erro!(err))?;
  Ok(result)
}

pub fn ask_bool(message: &str) -> Result<bool, RubxError> {
  dbg_call!(message);
  print!("{} (y/N) ", message);
  std::io::stdout().flush().map_err(|err| dbg_erro!(err))?;
  let mut buffer = String::new();
  std::io::stdin()
    .read_line(&mut buffer)
    .map_err(|err| dbg_erro!(err))?;
  let result = buffer.trim().to_lowercase();
  Ok(result == "y" || result == "yes" || result == "t" || result == "true" || result == "1")
}

pub fn len(text: &str) -> usize {
  dbg_call!(text);
  text.len()
}

pub fn del(text: &str, start: usize, end: usize) -> String {
  dbg_call!(text, start, end);
  let mut start = start;
  let mut end = end;
  if start > text.len() {
    start = text.len();
  }
  if end < start {
    end = start;
  }
  let mut result = String::new();
  for (i, c) in text.chars().enumerate() {
    if i < start || i >= end {
      result.push(c);
    }
  }
  result
}

pub fn del_rex(text: &str, regex: &str) -> RubxResult<String> {
  dbg_call!(text, regex);
  let regex = Regex::new(regex).map_err(|err| dbg_erro!(err))?;
  dbg_reav!(Ok(del_regex(text, regex)));
}

pub fn del_regex(text: &str, regex: Regex) -> String {
  dbg_call!(text, regex);
  dbg_reav!(regex.replace_all(text, "").to_string());
}

pub fn trim(text: &str) -> String {
  dbg_call!(text);
  String::from(text.trim())
}

pub fn is_empty(text: &str) -> bool {
  dbg_call!(text);
  text.is_empty()
}

pub fn is_ascii(text: &str) -> bool {
  dbg_call!(text);
  text.is_ascii()
}

pub fn is_equals(text: &str, with: &str) -> bool {
  dbg_call!(text, with);
  dbg_reav!(text == with)
}

pub fn is_equally(text: &str, with: &str) -> bool {
  dbg_call!(text, with);
  dbg_reav!(text.to_lowercase() == with.to_lowercase())
}

pub fn is_likely(text: &str, with: &str) -> bool {
  dbg_call!(text, with);
  let mut side_a = text.chars();
  let mut side_b = with.chars();
  while let Some(item_a) = side_a.next() {
    if !item_a.is_whitespace() {
      let mut checked = false;
      let test_a = item_a.to_lowercase().to_string();
      dbg_tell!(test_a);
      while let Some(item_b) = side_b.next() {
        if !item_b.is_whitespace() {
          let test_b = item_b.to_lowercase().to_string();
          dbg_tell!(test_b);
          checked = test_a == test_b;
          break;
        }
      }
      if !checked {
        dbg_reav!(false);
      }
    }
  }
  dbg_reav!(true)
}

pub fn is_whitespace(text: &str) -> bool {
  dbg_call!();
  dbg_reav!(!text.chars().any(|ch| !ch.is_whitespace()))
}

pub fn is_linespace(text: &str) -> bool {
  dbg_call!();
  dbg_reav!(!text
    .chars()
    .any(|ch| LINE_SPACE_CHARS.iter().any(|item| ch != *item)))
}

pub fn is_linebreak(text: &str) -> bool {
  dbg_call!();
  dbg_reav!(!text
    .chars()
    .any(|ch| LINE_BREAK_CHARS.iter().any(|item| ch != *item)))
}

pub fn is_brackets(text: &str) -> bool {
  dbg_call!();
  dbg_reav!(!text
    .chars()
    .any(|ch| BRACKETS_CHARS.iter().any(|item| ch != *item)))
}

pub fn is_quotation(text: &str) -> bool {
  dbg_call!();
  dbg_reav!(!text
    .chars()
    .any(|ch| QUOTATION_CHARS.iter().any(|item| ch != *item)))
}

pub fn tolower(text: &str) -> String {
  dbg_call!(text);
  String::from(text.to_lowercase())
}

pub fn toupper(text: &str) -> String {
  dbg_call!(text);
  String::from(text.to_uppercase())
}

pub fn tocapital(text: &str) -> String {
  dbg_call!(text);
  if text.is_empty() {
    return String::default();
  }
  let mut result = text[0..1].to_uppercase();
  if text.len() > 1 {
    result.push_str(text[1..].to_lowercase().as_ref());
  }
  dbg_reav!(result)
}

pub fn contains(text: &str, part: &str) -> bool {
  dbg_call!(text, part);
  text.contains(part)
}

pub fn find(text: &str, part: &str) -> Option<usize> {
  dbg_call!(text, part);
  text.find(part)
}

pub fn rfind(text: &str, part: &str) -> Option<usize> {
  dbg_call!(text, part);
  text.rfind(part)
}

pub fn starts_with(text: &str, prefix: &str) -> bool {
  dbg_call!(text, prefix);
  text.starts_with(prefix)
}

pub fn ends_with(text: &str, suffix: &str) -> bool {
  dbg_call!(text, suffix);
  text.ends_with(suffix)
}

pub fn split(text: &str, pattern: &str) -> Vec<String> {
  dbg_call!(text, pattern);
  text.split(pattern).map(|item| item.to_string()).collect()
}

pub fn split_spaces(text: &str) -> Vec<String> {
  dbg_call!(text);
  text
    .split_whitespace()
    .map(|item| item.to_string())
    .collect()
}

pub fn text_file_find(path: &str, content: String) -> Result<Option<Vec<String>>, RubxError> {
  dbg_call!(path, content);
  text_file_find_any(path, vec![content])
}

pub fn text_file_find_any(
  path: &str,
  contents: Vec<String>,
) -> Result<Option<Vec<String>>, RubxError> {
  dbg_call!(path, contents);
  let mut results: Option<Vec<String>> = None;
  let file = File::open(path).map_err(|err| dbg_erro!(err))?;
  let mut reader = BufReader::new(file);
  let mut line = String::new();
  let mut row = 1;
  let mut done = 0;
  loop {
    line.clear();
    if reader.read_line(&mut line).map_err(|err| dbg_erro!(err))? == 0 {
      break;
    }
    for content in &contents {
      if let Some(col) = line.find(content) {
        if results.is_none() {
          results = Some(Vec::new());
        }
        let pos = done + col;
        let len = content.len();
        results
          .as_mut()
          .ok_or("Could not get the results as mutable")
          .map_err(|err| dbg_erro!(err))?
          .push(format!(
            "({})[{},{},{},{}]{}",
            path,
            row,
            col,
            pos,
            len,
            line.trim()
          ));
      }
    }
    done = done + line.len();
    row += 1;
  }
  Ok(results)
}

pub fn text_files_find(
  paths: Vec<String>,
  content: String,
) -> Result<Option<Vec<String>>, RubxError> {
  dbg_call!(paths, content);
  text_files_find_any(paths, vec![content])
}

pub fn text_files_find_any(
  paths: Vec<String>,
  contents: Vec<String>,
) -> Result<Option<Vec<String>>, RubxError> {
  dbg_call!(paths, contents);
  let cpus = num_cpus::get();
  let pool = Arc::new(Mutex::new(paths));
  let mut handles: Vec<JoinHandle<Option<Vec<String>>>> = Vec::with_capacity(cpus);
  for _ in 0..cpus {
    let link_pool = pool.clone();
    let link_contents = contents.clone();
    let handle = std::thread::spawn(move || -> Option<Vec<String>> {
      let mut partial: Option<Vec<String>> = None;
      loop {
        let path = {
          let mut lock_pool = link_pool.lock().map_err(|err| dbg_erro!(err)).unwrap();
          lock_pool.pop()
        };
        if path.is_none() {
          break;
        }
        let path = path.unwrap();
        let file_founds = text_file_find_any(&path, link_contents.clone())
          .map_err(|err| dbg_erro!(err))
          .unwrap();
        if let Some(file_founds) = file_founds {
          if partial.is_none() {
            partial = Some(Vec::new());
          }
          let edit_partial = partial
            .as_mut()
            .ok_or("Could not get partial results as mutable")
            .map_err(|err| dbg_erro!(err))
            .unwrap();
          for found in file_founds {
            edit_partial.push(found);
          }
        }
      }
      partial
    });
    handles.push(handle);
  }
  let mut results: Option<Vec<String>> = None;
  for handle in handles {
    let partial = match handle.join() {
      Ok(partial) => partial,
      Err(error) => return Err(dbg_erro!(format!("{:?}", error))),
    };
    if let Some(partial) = partial {
      if results.is_none() {
        results = Some(Vec::new());
      }
      let editor = results
        .as_mut()
        .ok_or("Could not get results as mutable")
        .map_err(|err| dbg_erro!(err))
        .unwrap();
      for found in partial {
        editor.push(found);
      }
    }
  }
  Ok(results)
}

pub fn text_file_founds(found: &str) -> Vec<String> {
  dbg_call!(found);
  let mut result: Vec<String> = Vec::new();
  let mut actual = String::new();
  let mut first = true;
  for ch in found.chars() {
    if first {
      first = false;
      continue;
    }
    if result.len() == 0 {
      if ch == ')' {
        result.push(actual.clone());
        actual.clear();
      } else {
        actual.push(ch);
      }
    } else if result.len() > 0 && result.len() < 5 {
      if ch == ',' || ch == ']' {
        result.push(actual.clone());
        actual.clear();
      }
      if ch.is_numeric() {
        actual.push(ch);
      }
    } else {
      actual.push(ch);
    }
  }
  result.push(actual);
  result
}

pub fn read(path: &str) -> Result<String, RubxError> {
  dbg_call!(path);
  let mut file = std::fs::OpenOptions::new()
    .create(false)
    .write(false)
    .read(true)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  let mut result = String::new();
  file
    .read_to_string(&mut result)
    .map_err(|err| dbg_erro!(err, path))?;
  Ok(result)
}

pub fn write(path: &str, contents: String) -> Result<(), RubxError> {
  dbg_call!(path, contents);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .append(false)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  Ok(write!(file, "{}", contents).map_err(|err| dbg_erro!(err, path))?)
}

pub fn write_lines(path: &str, lines: Vec<String>) -> Result<(), RubxError> {
  dbg_call!(path, lines);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .append(false)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  for line in lines {
    writeln!(file, "{}", line).map_err(|err| dbg_erro!(err, path, line))?;
  }
  Ok(())
}

pub fn write_inputs(path: &str) -> Result<(), RubxError> {
  dbg_call!(path);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .append(false)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  let input = std::io::stdin();
  let mut buffer = String::new();
  loop {
    let size_read = input
      .read_line(&mut buffer)
      .map_err(|err| dbg_erro!(err, path))?;
    if size_read == 0 {
      break;
    }
    write!(file, "{}", buffer).map_err(|err| dbg_erro!(err, path, buffer))?;
    buffer.clear();
  }
  dbg_reav!(Ok(()));
}

pub fn append(path: &str, contents: String) -> Result<(), RubxError> {
  dbg_call!(path, contents);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(false)
    .append(true)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  Ok(writeln!(file, "{}", contents).map_err(|err| dbg_erro!(err, path))?)
}

pub fn append_lines(
  path: &str,
  lines: &[impl AsRef<str> + std::fmt::Debug],
) -> Result<(), RubxError> {
  dbg_call!(path, lines);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(false)
    .append(true)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  for line in lines {
    writeln!(file, "{}", line.as_ref()).map_err(|err| dbg_erro!(err, path, line))?;
  }
  dbg_reav!(Ok(()));
}

pub fn append_inputs(path: &str) -> Result<(), RubxError> {
  dbg_call!(path);
  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(false)
    .append(true)
    .open(path)
    .map_err(|err| dbg_erro!(err, path))?;
  let input = std::io::stdin();
  let mut buffer = String::new();
  loop {
    let size_read = input
      .read_line(&mut buffer)
      .map_err(|err| dbg_erro!(err, path))?;
    if size_read == 0 {
      break;
    }
    write!(file, "{}", buffer).map_err(|err| dbg_erro!(err, path, buffer))?;
    buffer.clear();
  }
  dbg_reav!(Ok(()));
}

pub fn find_bigger_line(on: &[impl AsRef<str> + std::fmt::Debug]) -> Option<&str> {
  dbg_call!(on);
  if on.is_empty() {
    dbg_reav!(None);
  }
  let mut bigger = on[0].as_ref();
  for line in on {
    if line.as_ref().len() > bigger.len() {
      bigger = line.as_ref();
    }
  }
  dbg_reav!(Some(bigger));
}

pub fn find_smaller_line(on: &[impl AsRef<str> + std::fmt::Debug]) -> Option<&str> {
  dbg_call!(on);
  if on.is_empty() {
    dbg_reav!(None);
  }
  let mut smallest = on[0].as_ref();
  for line in on {
    if line.as_ref().len() < smallest.len() {
      smallest = line.as_ref();
    }
  }
  dbg_reav!(Some(smallest));
}

pub fn read_setup(path: &str) -> Result<HashMap<String, String>, RubxError> {
  dbg_call!(path);
  let file = File::open(path).map_err(|err| dbg_erro!(err, path))?;
  let reader = BufReader::new(file);
  let mut result = HashMap::new();
  for line in reader.lines() {
    let line = line.map_err(|err| dbg_erro!(err, path))?;
    if line.starts_with("#") || line.is_empty() {
      continue;
    }
    let equals_pos = line.find('=');
    if let Some(equals_pos) = equals_pos {
      let key = line[0..equals_pos].trim().to_string();
      let value = line[equals_pos + 1..].trim().to_string();
      result.insert(key, value);
    } else {
      result.insert(line.clone(), String::new());
    }
  }
  dbg_reav!(Ok(result));
}

pub fn is_truthy(value: &str) -> bool {
  dbg_call!(value);
  if value.is_empty() {
    return false;
  }
  let value = value.to_lowercase();
  if value == "true"
    || value == "t"
    || value == "yes"
    || value == "y"
    || value == "on"
    || value == "1"
  {
    dbg_reav!(true);
  } else {
    dbg_reav!(false);
  }
}
