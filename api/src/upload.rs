use async_std::path::Path;

use salvo::{http::form::FilePart, Response};

pub async fn upload(file: Option<&FilePart>, res: &mut Response) {
    let file = file;
    match file {
        Some(file) => {
            let dest = format!("temp/{}", file.name().unwrap_or("file"));
            let _info: Result<u64, std::io::Error> = std::fs::copy(&file.path(), Path::new(&dest));
        }
        None => {} // empty bloc cause we don't have any file.
    }
}
