use backtrace::{resolve, trace};
use colored::*;

/// This function prints the current stack frame.
pub fn print_stackframe() {
    print!("Begin {}", "Stack Trace:".bold());
    let mut frame_cnt = 0;
    // Inspects the current call-stack, passing all active frames into the closure
    // provided to calculate a stack trace.
    trace(|frame| {
        // The instruction pointer (IP) of the frame.
        let ip = frame.ip();
        println!("\n{} #{:<02} -> {:#03$X}",
            "Stack Num".red().italic().underline(), frame_cnt, ip as usize, 12
        );
        frame_cnt += 1;

        // Resolves the symbol information for the given instruction pointer.
        resolve(ip, |symbol| {
            // The symbol name.
            if let Some(name) = symbol.name() {
                println!("{} {}",
                    "- Func:".magenta(), name
                );
            }
            // The symbol filename and line number.
            if let Some(file) = symbol.filename() {
                if let Some(l) = symbol.lineno() {
                    println!("{} {}:{}",
                        "@ File:".cyan(), file.display(), l
                    );
                }
            }
        });
        // Return true to continue the stack trace, or false to stop.
        true
    });
}
