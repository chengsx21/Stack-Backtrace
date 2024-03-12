/// This example demonstrates how to print the current stack frame.
use stack_backtrace::print_stackframe;

fn bar() {
    print_stackframe()
}

fn foo() {
    bar()
}

fn main() {
    foo();
}