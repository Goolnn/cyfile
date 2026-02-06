// use crate::Codec;
// use crate::Project;
// use crate::codec::Writer;
// use crate::file;
// use crate::file::Manifest;
// use crate::project::Asset;
// use std::collections::HashSet;
// use std::fs::File;
// use std::io::Seek;
// use std::io::Write;
// use std::path::Path;
// use zip::CompressionMethod;
// use zip::ZipWriter;
// use zip::write::SimpleFileOptions;

// const IDENTIFIER: &[u8] = b"Cangyan Project Package";

// pub fn save_to_path<P: AsRef<Path>>(
//     project: &mut Project,
//     manifest: Manifest,
//     path: P,
// ) -> file::Result<()> {
//     let path = path.as_ref();

//     if path.exists() && !path.is_file() {
//         return Err(file::Error::PathNotFile {
//             path: path.to_path_buf(),
//         });
//     }

//     let file = match File::create(path) {
//         Ok(val) => val,

//         Err(err) => match err.kind() {
//             std::io::ErrorKind::PermissionDenied => {
//                 return Err(file::Error::PermissionDenied {
//                     path: path.to_path_buf(),
//                 });
//             }

//             std::io::ErrorKind::NotFound => {
//                 return Err(file::Error::PathNotExist {
//                     path: path.to_path_buf(),
//                 });
//             }

//             _ => {
//                 return Err(file::Error::Undefined);
//             }
//         },
//     };

//     save_to_stream(project, manifest, file)
// }

// pub fn save_to_stream<W: Write + Seek>(
//     project: &mut Project,
//     manifest: Manifest,
//     writer: W,
// ) -> file::Result<()> {
//     let mut archive = ZipWriter::new(writer);

//     archive.set_comment(String::from_utf8_lossy(IDENTIFIER).to_string());

//     write_json_file(&mut archive, "cangyan.json", &manifest)?;

//     let mut writer = Writer::new(manifest);
//     project.encode(&mut writer)?;
//     write_json_file(&mut archive, "project.json", &writer.into_value())?;

//     write_assets(&mut archive, project)?;

//     archive.finish().map_err(|_| file::Error::Undefined)?;

//     Ok(())
// }

// fn write_json_file<W: Write + Seek, T: serde::Serialize>(
//     archive: &mut ZipWriter<W>,
//     name: &str,
//     value: &T,
// ) -> file::Result<()> {
//     let data = serde_json::to_vec(value).map_err(|_| file::Error::Undefined)?;

//     let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

//     archive
//         .start_file(name, options)
//         .map_err(|_| file::Error::Undefined)?;

//     archive.write_all(&data).map_err(|_| file::Error::Undefined)
// }

// fn write_assets<W: Write + Seek>(
//     archive: &mut ZipWriter<W>,
//     project: &mut Project,
// ) -> file::Result<()> {
//     let mut seen = HashSet::new();

//     write_asset(archive, project.cover_mut(), &mut seen)?;

//     for page in project.pages_mut() {
//         write_asset(archive, page.image_mut(), &mut seen)?;
//     }

//     Ok(())
// }

// fn write_asset<W: Write + Seek>(
//     archive: &mut ZipWriter<W>,
//     asset: &mut Asset,
//     seen: &mut HashSet<String>,
// ) -> file::Result<()> {
//     let path = asset.path().to_string();

//     if !seen.insert(path.clone()) {
//         return Ok(());
//     }

//     let data = asset.load()?;

//     let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

//     archive
//         .start_file(&path, options)
//         .map_err(|_| file::Error::Undefined)?;

//     archive.write_all(data).map_err(|_| file::Error::Undefined)
// }
