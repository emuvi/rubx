fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  let mut index = 0;
  while index < args.len() {
    let arg = &args[index];
    if arg == "--help" || arg == "-h" {
      help();
      return;
    } else if arg == "--call" || arg == "-c" {
      call(&args, index + 1, &mut index);
    }
    index += 1;
  }
}

fn help() {
  println!("{}", "Help usage");
}

fn call(args: &Vec<String>, start_at: usize, update_on: &mut usize) {
  if start_at >= args.len() {
    eprintln!("No module name was specified to call");
    return;
  }
  let module = &args[start_at];
  *update_on += 1;
  call_module(module, args, start_at + 1, update_on);
}

fn call_module(module: &str, args: &Vec<String>, start_at: usize, update_on: &mut usize) {
  if start_at >= args.len() {
    eprintln!(
      "No function name for the module {} was specified to call",
      module
    );
    return;
  }
  let function = &args[start_at];
  *update_on += 1;
  call_function(module, function, args, start_at + 1, update_on);
}

fn call_function(
  module: &str,
  function: &str,
  args: &Vec<String>,
  start_at: usize,
  update_on: &mut usize,
) {
  match module {
    "rands" => match function {
      "range" => {
        let min = args[start_at].parse::<u32>().unwrap();
        let max = args[start_at + 1].parse::<u32>().unwrap();
        println!("{}", rubx::rux_rands::range(min, max));
        *update_on += 2;
      }
      "chars" => {
        let count = args[start_at].parse::<usize>().unwrap();
        println!("{}", rubx::rux_rands::chars(count));
        *update_on += 1;
      }
      _ => {
        eprintln!(
          "The function {} of module {} is not supported",
          function, module
        );
      }
    },
    _ => {
      eprintln!("The module {} is not supported", module);
    }
  }
}
