use self_update::cargo_crate_version;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("divanvisagie")
        .repo_name("chat-gipitty")
        .bin_name("cgip")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Updated to version: {}", status.version());
    Ok(())
}
