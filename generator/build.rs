fn main() -> anyhow::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let clone_dir = out_dir.join("nerd-fonts");
    if clone_dir.exists() {
        return Ok(());
    }
    let status = std::process::Command::new("git")
        .args([
            "clone",
            "--branch=v3.1.1",
            "--filter=blob:none",
            "--depth=1",
            "--sparse",
            "https://github.com/ryanoasis/nerd-fonts",
        ])
        .arg(clone_dir)
        .status()?;
    if !status.success() {
        anyhow::bail!("clone failed");
    }
    Ok(())
}
