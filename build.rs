use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    build_shaders()?;
    move_res()?;
    Ok(())
}
fn move_res() -> Result<()> {
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["res/"];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;
    Ok(())
}
fn build_shaders() -> Result<()> {
    println!("cargo:rerun-if-changed=src/client/shaders/*");

    let shader_dir = "src/client/shaders";
    let out_dir = "src/client/compiled_shaders";

    println!("cargo:rerun-if-changed={shader_dir}");

    std::fs::create_dir_all(out_dir)?;

    let compiler = wesl::Wesl::new(shader_dir);

    for entry in walkdir::WalkDir::new(shader_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("wesl") {continue;}

        let relative = path.strip_prefix(shader_dir).unwrap();
        let module_path = relative
            .with_extension("")
            .components()
            .map(|c| c.as_os_str().to_str().unwrap())
            .collect::<Vec<_>>()
            .join("::");
        let module_path = format!("package::{module_path}");

        let wgsl = match compiler.compile(&module_path.parse().unwrap()) {
            Result::Ok(result) => result.to_string(),
            Err(e) => {
                eprintln!("WESL error in {}: {e}", path.display());
                panic!()
            }
        };

        let out_path = Path::new(out_dir)
            .join(relative)
            .with_extension("wgsl");

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        std::fs::write(&out_path, wgsl).unwrap();
        println!("cargo:warning=Compiled {} -> {}", path.display(), out_path.display());
    }
    Ok(())
}