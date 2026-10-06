use crate::config::Config;
use crate::container::ContainerArtifacts;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::CompressionMethod::Deflated;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn create_release_zip(
    config: &Config,
    containers: &ContainerArtifacts,
    zip_name: &str,
) -> Result<PathBuf> {
    std::fs::create_dir_all(&config.dist_dir)
        .with_context(|| format!("Failed to create dist dir at {:?}", config.dist_dir))?;

    let zip_path = config.dist_dir.join(format!("{}.zip", zip_name));
    let file = File::create(&zip_path)
        .with_context(|| format!("Failed to create zip file at {:?}", zip_path))?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(Deflated);

    // 1. Add IoStore container files
    for path in [&containers.pak_path, &containers.utoc_path, &containers.ucas_path] {
        let file_name = path
            .file_name()
            .context("Invalid container file name")?
            .to_string_lossy();
        zip.start_file(file_name, options)?;
        let bytes = std::fs::read(path)?;
        zip.write_all(&bytes)?;
    }

    // 2. Add UE4SS Runtime mod directory if present
    if config.runtime_mod_src.exists() {
        add_dir_to_zip(&mut zip, &config.runtime_mod_src, Path::new("Mods/CompleteStory"), options)?;
    }

    zip.finish().context("Failed to finalize zip archive")?;
    Ok(zip_path)
}

fn add_dir_to_zip(
    zip: &mut ZipWriter<File>,
    src_dir: &Path,
    archive_prefix: &Path,
    options: SimpleFileOptions,
) -> Result<()> {
    for entry in std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let rel_archive_path = archive_prefix.join(file_name);
        let archive_entry_name = rel_archive_path.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            zip.add_directory(format!("{}/", archive_entry_name), options)?;
            add_dir_to_zip(zip, &path, &rel_archive_path, options)?;
        } else {
            zip.start_file(archive_entry_name, options)?;
            let bytes = std::fs::read(&path)?;
            zip.write_all(&bytes)?;
        }
    }
    Ok(())
}
