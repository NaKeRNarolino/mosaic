use std::fs;
use subprocess::{Exec, NullFile};
use crate::web;

pub fn create_required_folders() -> anyhow::Result<()> {
    let _ = fs::create_dir_all("./mosaic_libs");

    Ok(())
}

pub async fn download(name: String) -> anyhow::Result<()> {
    create_required_folders()?;

    let index = web::get_index().await?;

    // dbg!(&index);

    let p = index.get(&name).unwrap();

    let _ = fs::remove_dir_all(format!("./mosaic_libs/{name}"));

    let e = Exec::cmd("git").arg("clone").arg(
        &p.git
    ).arg(format!("./mosaic_libs/{name}"));

    e.stdout(NullFile).capture()?;

    // let e = Exec::cmd("rm").arg("-rf").arg(format!("./mosaic_libs/{name}/.git"));
    // e.capture()?;
    let _ = fs::remove_dir_all(format!("./mosaic_libs/{name}/.git"));

    Ok(())
}