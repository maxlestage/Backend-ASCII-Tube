use async_std::path::Path;

use salvo::{
    prelude::StatusCode,
    writer::Text,
    {Request, Response},
};

pub async fn upload(req: &mut Request, res: &mut Response) {
    // select_user_by_email(jwt) -> {id mail etc ....}

    let file = req.first_file().await; // chrislearn — 29/03/2023 à 09:18 use req.first_file().await to get the uploaded file instead of req.file("").await
    if let Some(file) = file {
        let dest = format!("temp/{}", file.name().unwrap_or("file"));
        let info = if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            dest
        } else {
            dest
        };
    }
}
