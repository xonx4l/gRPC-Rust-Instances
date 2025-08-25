fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/greeter.proto", "proto/chatter/proto"],
            &["proto/"],
        )?;
        Ok(())
} 