use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use clap::Parser;
use zip::{read::ZipArchive, ZipWriter};
use zip::CompressionMethod::Stored;
use zip::write::FileOptions;

fn unzip(source: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(source)?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let output = Path::new(target).join(file.mangled_name());
        if file.is_dir() {
            std::fs::create_dir_all(&output)?;
        } else {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut output_file = File::create(&output)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }
    Ok(())
}

fn zip(source: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(target)?;
    let options = FileOptions::default().compression_method(Stored);
    let mut zip = ZipWriter::new(file);
    add_files(&mut zip, source, source, &options)?;
    Ok(())
}

fn add_files(zip: &mut ZipWriter<File>, source: &Path, current: &Path, options: &FileOptions) -> Result<(), Box<dyn std::error::Error>> {
    let files = std::fs::read_dir(current)?;
    for file in files {
        let file = file?;
        let path = file.path();
        let name = path.strip_prefix(source)?;
        if path.is_dir() {
            zip.add_directory(name.to_string_lossy(), *options)?;
            add_files(zip, source, &path, options)?;
        } else {
            zip.start_file(name.to_string_lossy(), *options)?;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, ignore_errors = true)]
struct Args {
    #[arg(short, long)]
    create: bool,

    #[arg(short, long)]
    zip: String,

    #[arg(short, long, default_value = ".")]
    dir: String,
}

fn main() {
    let args = Args::parse();
    if args.create {
        let source = args.dir;
        let target = args.zip;
        println!("开始压缩!");
        match zip(Path::new(&source), Path::new(&target)) {
            Ok(()) => println!("压缩成功!"),
            Err(err) => eprintln!("压缩失败: {}", err),
        }
    } else {
        let source = args.zip;
        let target = args.dir;
        println!("开始解压!");
        match unzip(&source, &target) {
            Ok(()) => println!("解压成功!"),
            Err(err) => eprintln!("解压失败: {}", err),
        }
    }
}