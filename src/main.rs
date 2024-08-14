use std::ffi::CString;
use std::fs;
use std::io::{stdout, Read, Write};
use std::path::Path;

fn main() {
    //List files
    let mut files: Vec<(Box<Path>, Box<Path>)> = vec![]; //(Real path, Embedded path)

    let mut processing_heap: Vec<(Box<Path>, Box<Path>)> = vec![]; //(Real path, embedded path)
    for elem in std::env::args().skip(1) {
        let real_path = Path::new(&elem);
        let sym_path = match real_path.file_name() {
            None => {
                continue; //Skip invalid files
            }
            Some(x) => Path::new(x),
        };

        processing_heap.push((Box::from(real_path), Box::from(sym_path)));
    }

    while let Some((real_path, embedded_path)) = processing_heap.pop() {
        if let Ok(true) = real_path.try_exists() {
        } else {
            eprint!(
                "File/Directory {} does not exist. Skipping.",
                real_path.to_string_lossy()
            );
            continue;
        }

        if real_path.is_file() {
            files.push((real_path, embedded_path));
            continue;
        }

        if real_path.is_dir() {
            for entry in fs::read_dir(real_path).expect("Error reading dirs") {
                let entry = entry.expect("Error reading dirs 2");

                let path = entry.path();
                let new_sympath = embedded_path.join(path.file_name().unwrap());

                processing_heap.push((Box::from(path), Box::from(new_sympath)));
            }
        }
    }

    //Process files
    let queried: Vec<(Box<Path>, Box<Path>, u64)> = files
        .into_iter()
        .map(|(real, sym)| (real.clone(), sym, fs::metadata(real.clone()).unwrap().len()))
        .collect();

    //Position of the writing cursor, in bytes
    let mut write_index: u64 = 0;

    //Compute size of header, in bytes
    let mut header_size: u64 = 8; //Start at 8 because header_size is stored at the start of the header
    for (_, sym, size) in &queried {
        header_size += 8 * 2; //Sizeof u64 * 2
        header_size += CString::new(&*sym.to_string_lossy())
            .unwrap()
            .as_bytes_with_nul()
            .len() as u64;
    }

    //Write header size
    stdout().write_all(&header_size.to_le_bytes()).unwrap();

    //Write index
    for (_, sym, size) in &queried {
        stdout()
            .write_all(&size.to_le_bytes())
            .expect("Write error ! Aborting");
        stdout()
            .write_all(&(write_index + header_size).to_le_bytes())
            .expect("Write error ! Aborting");

        let strpath: &str = &*sym.to_string_lossy();
        let cpath = CString::new(strpath)
            .expect("Failed converting the path into a valid C string. Aborting");
        stdout()
            .write_all(cpath.as_bytes_with_nul())
            .expect("Write error ! Aborting");

        write_index += size;
    }

    //Write files
    for (real, _, _) in &queried {
        stdout()
            .write_all(&*fs::read(real).unwrap())
            .expect("Write error ! Aborting");
    }
}
