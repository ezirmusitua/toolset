use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use zip::write::FileOptions;

fn read_dir(path: &Path, predicate: fn(&DirEntry) -> bool) -> Result<Vec<DirEntry>, String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    if !path.is_dir() {
        return Err(format!("{} not a directory", filename).to_string());
    }
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return Err("Read directory error".to_string()),
    };
    let mut candidates: Vec<DirEntry> = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        if predicate(&entry) {
            candidates.push(entry);
        }
    }
    return Ok(candidates);
}

fn ends_with_epub(entry: &DirEntry) -> bool {
    let filename = entry.file_name();
    let filename = filename.to_str().unwrap();
    return filename.ends_with(".epub");
}

fn copy_original(from: &Path, to: &Path) {
    let filename = from.file_name().unwrap().to_str();
    match fs::copy(from, to) {
        Ok(_) => println!("{} was copied", &filename.unwrap()),
        Err(e) => println!("Error while copy: {}", e),
    };
}

fn write_zip_file(
    pkg: &mut zip::ZipWriter<File>,
    path: &Path,
    folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filepath = {
        if folder == "" || folder == "/" {
            String::from(filename)
        } else {
            format!("{}/{}", folder, filename)
        }
    };
    pkg.start_file(filepath, options)?;
    let content = &fs::read(path)?;
    pkg.write_all(content)?;
    Ok(())
}

fn write_zip_directory(
    mut pkg: &mut zip::ZipWriter<File>,
    path: &Path,
    folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("write zip directory {}", path.display());
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let prefix: String = {
        if folder == "" {
            String::from("/")
        } else if folder == "/" {
            pkg.add_directory(filename, options)?;
            String::from(filename)
        } else {
            let prefix = format!("{}/{}", folder, filename);
            pkg.add_directory(&prefix, options)?;
            prefix
        }
    };
    println!("writing prefix, {}", prefix);
    let subs = read_dir(path, |_| true)?;
    for sub in subs {
        if sub.path().is_dir() {
            write_zip_directory(&mut pkg, &sub.path(), &prefix)?;
        } else {
            write_zip_file(&mut pkg, &sub.path(), &prefix)?;
        }
    }
    Ok(())
}

fn create_zip_archive(source: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "create zip archive {} -> {}",
        source.display(),
        dest.display()
    );
    let file = File::create(dest)?;
    let mut zip = zip::ZipWriter::new(file);
    write_zip_directory(&mut zip, source, "")?;
    zip.finish()?;
    Ok(())
}

fn main() {
    let source = Path::new("books");
    let dest = Path::new("packaged");
    let entries = match read_dir(source, ends_with_epub) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            exit(-1);
        }
    };
    for entry in entries {
        let filetype = entry.file_type().unwrap();
        let filename = entry.file_name();
        if filetype.is_dir() {
            match create_zip_archive(&entry.path(), &dest.join(filename)) {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            copy_original(&entry.path(), &dest.join(filename));
        }
    }
}
