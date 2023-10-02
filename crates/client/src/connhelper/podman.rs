use tokio::{io, process::Command};

use super::buildkit_stdio::BuildkitStdio;

pub(crate) async fn podman_connect(container: impl AsRef<str>) -> io::Result<BuildkitStdio> {
    let child = Command::new("podman")
        .arg("exec")
        .arg("-i")
        .arg(container.as_ref())
        .arg("buildctl")
        .arg("dial-stdio")
        .stdout(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    Ok(BuildkitStdio::new(child))
}
