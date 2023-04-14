use async_std::path::Path;

use salvo::{
    prelude::StatusCode,
    writer::Text,
    {Request, Response},
};

pub async fn upload(req: &mut Request, res: &mut Response) {
    match req.first_file().await {
        Some(file) => {
            let dest = format!("temp/{}", file.name().unwrap_or("file"));
            let _info: Result<u64, std::io::Error> = std::fs::copy(&file.path(), Path::new(&dest));
        }
        None => {}
    }
}
