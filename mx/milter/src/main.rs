//! AttachmentFilter milter. The protocol implementation lands in phase 1;
//! until then the binary only reports its version so packaging and CI have
//! something to build.

fn main() {
    println!(
        "afilter-milter {} (not yet functional)",
        env!("CARGO_PKG_VERSION")
    );
    std::process::exit(2);
}
