use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::debug::{dbg_bleb, dbg_erro};
use crate::debug::{dbg_call, dbg_reav, dbg_step};
use crate::paths;
use crate::RubxError;

pub fn cmd(
    command: &str,
    args: &[impl AsRef<str>],
    dir: Option<impl AsRef<str>>,
    print: Option<bool>,
    throw: Option<bool>,
) -> Result<(i32, String), RubxError> {
    dbg_call!(command, print, throw);
    let mut cmd = Command::new(command);
    let args = args
        .iter()
        .map(|arg| {
            let arg = arg.as_ref();
            cmd.arg(arg);
            arg.into()
        })
        .collect::<Vec<&str>>();
    dbg_step!(args);
    let dir: String = if let Some(dir) = dir {
        dir.as_ref().into()
    } else {
        ".".into()
    };
    dbg_step!(dir);
    cmd.current_dir(&dir);
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| dbg_erro!(err, command, args, dir))?;
    let mut output = String::new();
    child
        .stdout
        .take()
        .ok_or("Could not take on the child stdout")
        .map_err(|err| dbg_erro!(err))?
        .read_to_string(&mut output)
        .map_err(|err| dbg_erro!(err))?;
    dbg_step!(output);
    child
        .stderr
        .take()
        .ok_or("Could not take on the child stderr")
        .map_err(|err| dbg_erro!(err))?
        .read_to_string(&mut output)
        .map_err(|err| dbg_erro!(err))?;
    dbg_step!(output);
    let output = output.trim();
    let output = String::from(output);
    dbg_step!(output);
    let result = child
        .wait()
        .map_err(|err| dbg_erro!(err))?
        .code()
        .ok_or("Could not found the exit code")
        .map_err(|err| dbg_erro!(err))?;
    dbg_step!(result);
    let print = if let Some(print) = print { print } else { true };
    dbg_step!(print);
    if print && !output.is_empty() {
        println!("{}", output);
    }
    let throw = if let Some(throw) = throw { throw } else { true };
    dbg_step!(throw);
    if throw && result != 0 {
        return Err(dbg_erro!(
            "Result code from command is different than zero",
            command,
            result
        ));
    }
    Ok((result, output))
}

pub fn sleep(millis: u64) {
    dbg_call!(millis);
    thread::sleep(Duration::from_millis(millis))
}

pub fn pause() -> Result<(), RubxError> {
    dbg_call!();
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    write!(stdout, "Press enter to continue...").map_err(|err| dbg_erro!(err))?;
    stdout.flush()?;
    let mut buffer = [0u8];
    stdin.read(&mut buffer).map_err(|err| dbg_erro!(err))?;
    Ok(())
}

pub fn exe_path() -> Result<String, RubxError> {
    dbg_call!();
    dbg_reav!(Ok(format!(
        "{}",
        std::env::current_exe()
            .map_err(|err| dbg_erro!(err))?
            .display(),
    )));
}

pub fn exe_dir() -> Result<String, RubxError> {
    dbg_call!();
    let exe_path = exe_path().map_err(|err| dbg_bleb!(err))?;
    dbg_reav!(Ok(
        paths::path_parent(&exe_path).map_err(|err| dbg_bleb!(err))?,
    ));
}

pub fn exe_name() -> Result<String, RubxError> {
    dbg_call!();
    let exe_path = exe_path().map_err(|err| dbg_bleb!(err))?;
    dbg_reav!(Ok(paths::path_name(&exe_path).into()));
}

pub fn exe_stem() -> Result<String, RubxError> {
    dbg_call!();
    let exe_path = exe_path().map_err(|err| dbg_bleb!(err))?;
    dbg_reav!(Ok(paths::path_stem(&exe_path).into()));
}

pub fn exe_ext() -> &'static str {
    dbg_call!();
    dbg_reav!(std::env::consts::EXE_EXTENSION);
}

pub fn dot_exe_ext() -> String {
    dbg_call!();
    dbg_reav!(if std::env::consts::EXE_EXTENSION.is_empty() {
        String::default()
    } else {
        format!(".{}", std::env::consts::EXE_EXTENSION)
    });
}

pub fn get_os() -> &'static str {
    dbg_call!();
    dbg_reav!(std::env::consts::OS);
}

pub fn is_lin() -> bool {
    dbg_call!();
    dbg_reav!(std::env::consts::OS == "linux");
}

pub fn is_mac() -> bool {
    dbg_call!();
    dbg_reav!(std::env::consts::OS == "macos");
}

pub fn is_win() -> bool {
    dbg_call!();
    dbg_reav!(std::env::consts::OS == "windows");
}
