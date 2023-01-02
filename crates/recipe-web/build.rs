// #[cfg(feature = "embed")]
// #[tokio::main]
// async fn main() {
//     use tokio::io::{AsyncBufReadExt, BufReader};

//     println!("running trunk build");
//     let mut child = tokio::process::Command::new("trunk")
//         .arg("build")
//         .env("CARGO_TARGET_DIR", "trunk-target")
//         .spawn()
//         .expect("failed to start trunk build command");
//
//     if let Some(stdout) = child.stdout.take() {
//         tokio::spawn(async move {
//             let mut stdout = BufReader::new(stdout);
//             let mut line = String::new();
//             loop {
//                 line.clear();
//                 match stdout.read_line(&mut line).await {
//                     Err(err) => return Err(err),
//                     Ok(0) => return Ok(()),
//                     Ok(_) => {
//                         println!("{}", line);
//                     },
//                 }
//             }
//         });
//     }

//     if let Some(stderr) = child.stderr.take() {
//         tokio::spawn(async move {
//             let mut stderr = BufReader::new(stderr);
//             let mut line = String::new();
//             loop {
//                 line.clear();
//                 match stderr.read_line(&mut line).await {
//                     Err(err) => return Err(err),
//                     Ok(0) => return Ok(()),
//                     Ok(_) => {
//                         println!("{}", line);
//                     },
//                 }
//             }
//         });
//     }
//     child.wait().await.expect("failed to build assets");
// }

#[cfg(feature = "embed")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use trunk::{build, config};
    let b = config::ConfigOptsBuild {
        release: true,
        ..Default::default()
    };
    let cfg = config::ConfigOpts::rtc_build(b, None)?;
    let mut system = build::BuildSystem::new(cfg, None).await?;
    system.build().await?;
    Ok(())
}

#[cfg(not(feature = "embed"))]
fn main() {}
