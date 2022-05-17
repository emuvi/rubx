mod clip;

fn main() {
  let args = clip::parse();
  if args.is_present("verbose") {
    rubx::rux_debug::put_verbose();
  }
  if args.is_present("archive") {
    rubx::rux_debug::put_archive();
  }
  if args.is_present("debug-calls") {
    rubx::rux_debug::put_dbg_calls();
  }
  if args.is_present("debug-reavs") {
    rubx::rux_debug::put_dbg_reavs();
  }
  if args.is_present("debug-steps") {
    rubx::rux_debug::put_dbg_steps();
  }
  if args.is_present("debug-tells") {
    rubx::rux_debug::put_dbg_tells();
  }
}
