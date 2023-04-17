use async_std::path::Path;

use salvo::{http::form::FilePart, Response};

pub async fn upload(file: Option<&FilePart>, res: &mut Response) {
    let file = file; // chrislearn — 29/03/2023 à 09:18 use req.first_file().await to get the uploaded file instead of req.file("").await
    if let Some(file) = file {
        let dest = format!("temp/{}", file.name().unwrap_or("file"));
        let info = if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            dest
        } else {
            dest
        };
    }
}
