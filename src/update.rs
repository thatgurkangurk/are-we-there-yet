use anyhow::Result;
use self_update::cargo_crate_version;

pub fn update() -> Result<()> {
    let target = self_update::get_target();

    let asset_name = match target {
        "x86_64-pc-windows-gnu" => "are-we-there-yet-x86_64-pc-windows-gnu.exe",
        "x86_64-unknown-linux-gnu" => "are-we-there-yet-x86_64-unknown-linux-gnu",
        _ => panic!("Unsupported target {}", target),
    };

    let status = self_update::backends::github::Update::configure()
        .repo_owner("thatgurkangurk")
        .repo_name("are-we-there-yet")
        .bin_name("are-we-there-yet")
        .target(target)
        .bin_name(asset_name)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .show_download_progress(true)
        .show_output(true)
        .build()?
        .update()?;

    if status.updated() {
        println!("Updated to {}", status.version());
    } else {
        println!("Already up to date.");
    }

    Ok(())
}
