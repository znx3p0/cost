
fn main() {
    cc::Build::default()
        .file("perf.c")
        .compile("perf")
}
