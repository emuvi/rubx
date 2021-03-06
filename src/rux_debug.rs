use chrono::offset::Utc;
use once_cell::sync::Lazy;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::rux_texts;
use crate::rux_times;
use crate::RubxError;
use crate::RubxResult;

static VERBOSE: AtomicBool = AtomicBool::new(false);
static ARCHIVE: AtomicBool = AtomicBool::new(false);
static ARCFILE: Lazy<Mutex<File>> = Lazy::new(|| {
  let exe_path = match std::env::current_exe() {
    Ok(exe_path) => exe_path,
    Err(_) => PathBuf::from("archive.log"),
  };
  let exe_name = match exe_path.file_stem() {
    Some(exe_name) => format!("{}", exe_name.to_string_lossy()),
    None => String::from("archive"),
  };
  let arc_name = format!("{}.log", exe_name);
  let file = File::create(arc_name).expect("Could not create the archive file");
  Mutex::new(file)
});
static DBGTIME: AtomicBool = AtomicBool::new(false);
static DBGSIZE: AtomicUsize = AtomicUsize::new(1);

pub fn setup(from: &str) -> RubxResult<()> {
  let setup = rux_texts::read_setup(from).expect("Could not read the debug setup file.");
  for (key, value) in setup.iter() {
    match key.as_str() {
      "verbose" => set_verbose(rux_texts::is_truthy(&value)),
      "archive" => set_archive(rux_texts::is_truthy(&value)),
      "dbgtime" => set_dbg_time(rux_texts::is_truthy(&value)),
      "dbgsize" => set_dbg_size(
        value
          .parse::<usize>()
          .expect("Could not parse the debug size."),
      ),
      "debug-calls" => {
        if rux_texts::is_truthy(&value) {
          put_dbg_calls()
        }
      }
      "debug-reavs" => {
        if rux_texts::is_truthy(&value) {
          put_dbg_reavs()
        }
      }
      "debug-steps" => {
        if rux_texts::is_truthy(&value) {
          put_dbg_steps()
        }
      }
      "debug-tells" => {
        if rux_texts::is_truthy(&value) {
          put_dbg_tells()
        }
      }
      _ => (),
    }
  }
  Ok(())
}

pub fn is_verbose() -> bool {
  VERBOSE.load(Ordering::Acquire)
}

pub fn set_verbose(verbose: bool) {
  VERBOSE.store(verbose, Ordering::Release);
  if is_verbose() {
    dbg_info!("Verbose started");
  }
}

pub fn put_verbose() {
  set_verbose(true);
}

pub fn is_archive() -> bool {
  ARCHIVE.load(Ordering::Acquire)
}

pub fn set_archive(archive: bool) {
  ARCHIVE.store(archive, Ordering::Release);
  if is_archive() {
    dbg_info!("Archive started");
  }
}

pub fn put_archive() {
  set_archive(true);
}

pub fn is_dbg_time() -> bool {
  DBGTIME.load(Ordering::Acquire)
}

pub fn set_dbg_time(time: bool) {
  DBGTIME.store(time, Ordering::Release);
}

pub fn put_dbg_time() {
  set_dbg_time(true);
}

pub fn get_dbg_size() -> usize {
  DBGSIZE.load(Ordering::Acquire)
}

pub fn set_dbg_size(size: usize) {
  DBGSIZE.store(size, Ordering::Release)
}

pub fn put_dbg_calls() {
  set_dbg_size(1)
}

pub fn put_dbg_reavs() {
  set_dbg_size(2)
}

pub fn put_dbg_steps() {
  set_dbg_size(3)
}

pub fn put_dbg_tells() {
  set_dbg_size(4)
}

pub fn put_dbg_verbose_tells() {
  put_verbose();
  put_dbg_tells();
}

pub fn debug(message: impl AsRef<str>) {
  let thread_display = get_thread_display();
  if is_verbose() {
    if is_dbg_time() {
      println!(
        "{} - |{}| {}",
        Utc::now().format(rux_times::UNIQUE_REAL_FORMAT),
        &thread_display,
        message.as_ref()
      );
    } else {
      println!("|{}| {}", &thread_display, message.as_ref());
    }
  }
  if is_archive() {
    let mut file = ARCFILE.lock().unwrap();
    if is_dbg_time() {
      writeln!(
        file,
        "{} - |{}| {}",
        Utc::now().format(rux_times::UNIQUE_REAL_FORMAT),
        &thread_display,
        message.as_ref()
      )
      .unwrap();
    } else {
      writeln!(file, "|{}| {}", &thread_display, message.as_ref()).unwrap();
    }
  }
}

fn get_thread_display() -> String {
  format!(
    "{}:{}",
    &get_thread_id()[6..],
    std::thread::current().name().unwrap_or("")
  )
}

fn get_thread_id() -> String {
  format!("{:?}", &std::thread::current().id())
}

pub fn wrong(message: String) -> Box<MessageErr> {
  Box::new(MessageErr::of(message))
}

pub fn throw(message: String) -> RubxError {
  Box::new(MessageErr::of(message))
}

pub fn debug_info(
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> String {
  debug_of("INFO", file, line, func, vals, err)
}

pub fn debug_warn(
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> String {
  debug_of("WARN", file, line, func, vals, err)
}

pub fn debug_bleb(
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: RubxError,
) -> RubxError {
  let from = format!("{}", err);
  let from = if let Some(pos) = from.rfind(" on (") {
    &from[pos + 4..]
  } else {
    ""
  };
  let from = format!("<BLEB> from {}", from);
  debug_of("ERRO", file, line, func, vals, from);
  err
}

pub fn debug_erro(
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> RubxError {
  throw(debug_of("ERRO", file, line, func, vals, err))
}

pub fn debug_errs(
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> String {
  debug_of("ERRO", file, line, func, vals, err)
}

pub fn debug_jolt(
  kind: &str,
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> RubxError {
  throw(debug_of(kind, file, line, func, vals, err))
}

pub fn debug_kind(
  kind: &str,
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  err: impl Display,
) -> String {
  debug_of(kind, file, line, func, vals, err)
}

pub fn debug_call(file: &str, line: u32, func: &str, vals: String) {
  if get_dbg_size() >= 1 {
    debug_in("DBUG", "CALL", file, line, func, vals);
  }
}

pub fn debug_reav(file: &str, line: u32, func: &str, vals: String) {
  if get_dbg_size() >= 2 {
    debug_in("DBUG", "REAV", file, line, func, vals);
  }
}

pub fn debug_step(file: &str, line: u32, func: &str, vals: String) {
  if get_dbg_size() >= 3 {
    debug_in("DBUG", "STEP", file, line, func, vals);
  }
}

pub fn debug_lets(file: &str, line: u32, func: &str, what: &str, lets: String) {
  if get_dbg_size() >= 3 {
    debug_in(
      "DBUG",
      "LETS",
      file,
      line,
      func,
      format!("lets {} = {}", what, lets),
    );
  }
}

pub fn debug_muts(file: &str, line: u32, func: &str, what: &str, muts: String) {
  if get_dbg_size() >= 3 {
    debug_in(
      "DBUG",
      "MUTS",
      file,
      line,
      func,
      format!("muts {} = {}", what, muts),
    );
  }
}

pub fn debug_ifis(file: &str, line: u32, func: &str, what: &str, ifis: String) {
  if get_dbg_size() >= 4 {
    debug_in(
      "DBUG",
      "IFIS",
      file,
      line,
      func,
      format!("if is {} = {}", what, ifis),
    );
  }
}

pub fn debug_tell(file: &str, line: u32, func: &str, vals: String) {
  if get_dbg_size() >= 4 {
    debug_in("DBUG", "TELL", file, line, func, vals);
  }
}

pub fn debug_of(
  kind: &str,
  file: &str,
  line: u32,
  func: &str,
  vals: String,
  msg: impl Display,
) -> String {
  let message = if vals.is_empty() {
    format!("[{}] on ({}) in [{}:{}] {}", kind, func, file, line, msg)
  } else {
    format!(
      "[{}] on ({}) in [{}:{}] {} as {{ {} }}",
      kind, func, file, line, msg, vals
    )
  };
  debug(&message);
  message
}

pub fn debug_in(
  kind: &str,
  sub: &str,
  file: &str,
  line: u32,
  func: &str,
  vals: String,
) -> String {
  let message = format!(
    "[{}] <{}> on ({}) in [{}:{}] as {{ {} }}",
    kind, sub, func, file, line, vals
  );
  debug(&message);
  message
}

macro_rules! dbg_func {
  () => {{
    fn f() {}
    let name = crate::rux_debug::dbg_fnam!(f);
    &name[..name.len() - 3]
  }};
}

macro_rules! dbg_fnam {
  ($val:expr) => {{
    fn type_name_of<T>(_: T) -> &'static str {
      std::any::type_name::<T>()
    }
    type_name_of($val)
  }};
}

macro_rules! dbg_fval {
  ($v:expr) => {{
    let mut value = format!("{:?}", $v);
    if value.len() > 1000 {
      let mut end = 1000;
      while !value.is_char_boundary(end) {
        end += 1;
      }
      value.truncate(end);
    }
    value
  }};
}

macro_rules! dbg_fmts {
    () => (String::default());
    ($v:expr) => (format!("{}: {}", stringify!($v), crate::rux_debug::dbg_fval!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{}: {}, {}", stringify!($v), crate::rux_debug::dbg_fval!(&$v), crate::rux_debug::dbg_fmts!($($n),+)));
}

#[allow(unused_macros)]
macro_rules! dbg_fmsn {
  () => {
    String::default()
  };
  ($v:expr) => {
    format!("{}", crate::rux_debug::dbg_fval!(&$v))
  };
}

macro_rules! dbg_info {
    ($err:expr) => (
        crate::rux_debug::debug_info(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::rux_debug::debug_info(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $err)
    );
}

#[allow(unused_macros)]
macro_rules! dbg_warn {
    ($err:expr) => (
        crate::rux_debug::debug_warn(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::rux_debug::debug_warn(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $err)
    );
}

macro_rules! dbg_erro {
    ($err:expr) => (
        crate::rux_debug::debug_erro(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::rux_debug::debug_erro(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $err)
    );
}

#[allow(unused_macros)]
macro_rules! dbg_errs {
    ($err:expr) => (
        crate::rux_debug::debug_errs(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::rux_debug::debug_errs(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $err)
    );
}

macro_rules! dbg_bleb {
    ($err:expr) => (
        crate::rux_debug::debug_bleb(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        crate::rux_debug::debug_bleb(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $err)
    );
}

#[allow(unused_macros)]
macro_rules! dbg_jolt {
    ($kind:expr, $msg:expr) => (
        crate::rux_debug::debug_jolt($kind, file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::rux_debug::debug_jolt($kind, file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $msg)
    );
}

#[allow(unused_macros)]
macro_rules! dbg_kind {
    ($kind:expr, $msg:expr) => (
        crate::rux_debug::debug_kind($kind, file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        crate::rux_debug::debug_kind($kind, file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+), $msg)
    );
}

macro_rules! dbg_call {
    () => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_call(file!(), line!(), crate::rux_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_call(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+))
    );
}

macro_rules! dbg_reav {
  ($xp:expr) => {{
    let reav = $xp;
    #[cfg(debug_assertions)]
    crate::rux_debug::debug_reav(
      file!(),
      line!(),
      crate::rux_debug::dbg_func!(),
      crate::rux_debug::dbg_fmsn!(reav),
    );
    return reav;
  }};
}

macro_rules! dbg_step {
    () => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_step(file!(), line!(), crate::rux_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_step(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+))
    );
}

macro_rules! dbg_ifis {
  ($xp:expr) => {{
    let ifis = $xp;
    #[cfg(debug_assertions)]
    crate::rux_debug::debug_ifis(
      file!(),
      line!(),
      crate::rux_debug::dbg_func!(),
      stringify!($xp),
      crate::rux_debug::dbg_fval!(&ifis),
    );
    ifis
  }};
}

macro_rules! dbg_lets {
  ($xp:expr) => {{
    let lets = $xp;
    #[cfg(debug_assertions)]
    crate::rux_debug::debug_lets(
      file!(),
      line!(),
      crate::rux_debug::dbg_func!(),
      stringify!($xp),
      crate::rux_debug::dbg_fval!(&lets),
    );
    lets
  }};
}

macro_rules! dbg_muts {
  ($to:expr, $of:expr) => {{
    let muts = $of;
    #[cfg(debug_assertions)]
    crate::rux_debug::debug_muts(
      file!(),
      line!(),
      crate::rux_debug::dbg_func!(),
      stringify!($to),
      crate::rux_debug::dbg_fval!(&muts),
    );
    $to = muts;
  }};
}

macro_rules! dbg_tell {
    () => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_tell(file!(), line!(), crate::rux_debug::dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        crate::rux_debug::debug_tell(file!(), line!(), crate::rux_debug::dbg_func!(), crate::rux_debug::dbg_fmts!($($v),+))
    );
}

#[allow(unused_imports)]
pub(crate) use {dbg_bleb, dbg_erro, dbg_errs, dbg_info, dbg_jolt, dbg_kind, dbg_warn};
#[allow(unused_imports)]
pub(crate) use {dbg_call, dbg_ifis, dbg_lets, dbg_muts, dbg_reav, dbg_step, dbg_tell};
#[allow(unused_imports)]
pub(crate) use {dbg_fmsn, dbg_fmts, dbg_fnam, dbg_func, dbg_fval};

#[macro_export]
macro_rules! rux_dbg_fnam {
  ($val:expr) => {{
    fn type_name_of<T>(_: T) -> &'static str {
      std::any::type_name::<T>()
    }
    type_name_of($val)
  }};
}

#[macro_export]
macro_rules! rux_dbg_func {
  () => {{
    fn f() {}
    let name = rubx::rux_dbg_fnam!(f);
    &name[..name.len() - 3]
  }};
}

#[macro_export]
macro_rules! rux_dbg_fval {
  ($v:expr) => {{
    let mut value = format!("{:?}", $v);
    if value.len() > 1000 {
      let mut end = 1000;
      while !value.is_char_boundary(end) {
        end += 1;
      }
      value.truncate(end);
      value.push_str("...");
    }
    value
  }};
}

#[macro_export]
macro_rules! rux_dbg_fmts {
    () => (String::default());
    ($v:expr) => (format!("{}: {}", stringify!($v), rubx::rux_dbg_fval!(&$v)));
    ($v:expr, $($n:expr),+) => (format!("{}: {}, {}", stringify!($v), rubx::rux_dbg_fval!(&$v), rubx::rux_dbg_fmts!($($n),+)));
}

#[macro_export]
macro_rules! rux_dbg_fmsn {
  () => {
    String::default()
  };
  ($v:expr) => {
    format!("{}", rubx::rux_dbg_fval!(&$v))
  };
}

#[macro_export]
macro_rules! rux_dbg_info {
    ($msg:expr) => (
        rubx::rux_debug::debug_info(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_info(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_warn {
    ($msg:expr) => (
        rubx::rux_debug::debug_warn(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_warn(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_bleb {
    ($err:expr) => (
        rubx::rux_debug::debug_bleb(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $err)
    );
    ($err:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_bleb(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $err)
    );
}

#[macro_export]
macro_rules! rux_dbg_erro {
    ($msg:expr) => (
        rubx::rux_debug::debug_erro(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_erro(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_errs {
    ($msg:expr) => (
        rubx::rux_debug::debug_errs(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_errs(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_jolt {
    ($kind:expr, $msg:expr) => (
        rubx::rux_debug::debug_jolt($kind, file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_jolt($kind, file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_kind {
    ($kind:expr, $msg:expr) => (
        rubx::rux_debug::debug_kind($kind, file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!(), $msg)
    );
    ($kind:expr, $msg:expr, $($v:expr),+) => (
        rubx::rux_debug::debug_kind($kind, file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+), $msg)
    );
}

#[macro_export]
macro_rules! rux_dbg_call {
    () => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_call(file!(), line!(), rubx::rux_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_call(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+))
    );
}

#[macro_export]
macro_rules! rux_dbg_reav {
  ($xp:expr) => {{
    let reav = $xp;
    #[cfg(debug_assertions)]
    rubx::rux_debug::debug_reav(
      file!(),
      line!(),
      rubx::rux_dbg_func!(),
      rubx::rux_dbg_fmsn!(reav),
    );
    return reav;
  }};
}

#[macro_export]
macro_rules! rux_dbg_step {
    () => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_step(file!(), line!(), rubx::rux_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_step(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+))
    );
}

#[macro_export]
macro_rules! rux_dbg_ifis {
  ($xp:expr) => {{
    let ifis = $xp;
    #[cfg(debug_assertions)]
    rubx::rux_debug::debug_ifis(
      file!(),
      line!(),
      rubx::rux_dbg_func!(),
      stringify!($xp),
      rubx::rux_dbg_fval!(&ifis),
    );
    ifis
  }};
}

#[macro_export]
macro_rules! rux_dbg_lets {
  ($xp:expr) => {{
    let lets = $xp;
    #[cfg(debug_assertions)]
    rubx::rux_debug::debug_lets(
      file!(),
      line!(),
      rubx::rux_dbg_func!(),
      stringify!($xp),
      rubx::rux_dbg_fval!(&lets),
    );
    lets
  }};
}

#[macro_export]
macro_rules! rux_dbg_muts {
  ($to:expr, $of:expr) => {{
    let muts = $of;
    #[cfg(debug_assertions)]
    rubx::rux_debug::debug_muts(
      file!(),
      line!(),
      rubx::rux_dbg_func!(),
      stringify!($to),
      rubx::rux_dbg_fval!(&muts),
    );
    $to = muts;
  }};
}

#[macro_export]
macro_rules! rux_dbg_tell {
    () => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_tell(file!(), line!(), rubx::rux_dbg_func!(), String::default())
    );
    ($($v:expr),+) => (
        #[cfg(debug_assertions)]
        rubx::rux_debug::debug_tell(file!(), line!(), rubx::rux_dbg_func!(), rubx::rux_dbg_fmts!($($v),+))
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageErr {
  body: String,
}

impl MessageErr {
  #[inline]
  pub fn of(s: String) -> MessageErr {
    MessageErr { body: s }
  }

  #[inline]
  pub fn new<S: Into<String>>(s: S) -> MessageErr {
    MessageErr { body: s.into() }
  }

  #[inline]
  pub fn from<E: Error>(e: E) -> MessageErr {
    MessageErr {
      body: format!("{}", e),
    }
  }

  #[inline]
  pub fn with<E: Error>(s: &str, e: E) -> MessageErr {
    MessageErr {
      body: format!("{}, {}", s, e),
    }
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    &self.body
  }
}

impl Display for MessageErr {
  #[inline]
  fn fmt(&self, f: &mut Formatter) -> Result {
    self.body.fmt(f)
  }
}

impl Error for MessageErr {
  #[inline]
  fn description(&self) -> &str {
    &self.body
  }
}
