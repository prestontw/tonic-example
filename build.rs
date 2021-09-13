use std::env::{self, VarError};

fn main() -> Result<(), VarError> {
    let mut includes = vec!["proto".to_string()];
    let dir = env::var("GOOGLEAPIS_DIR")?;
    includes.push(dir);

    tonic_build::configure()
        .compile(&["proto/echo.proto".to_string()], &includes)
        .unwrap();
    Ok(())
}
