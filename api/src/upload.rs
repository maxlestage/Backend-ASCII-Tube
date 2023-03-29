use async_std::path::Path;
use salvo::{
    handler,
    prelude::StatusCode,
    writer::Text,
    {Request, Response},
};

#[handler]
pub async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.first_file().await; // chrislearn — 29/03/2023 à 09:18 use req.first_file().await to get the uploaded file instead of req.file("").await
    if let Some(file) = file {
        let dest = format!("temp/{}", file.name().unwrap_or("file"));
        let info = if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            format!("file not found in request: {}", e)
        } else {
            format!("File uploaded to {}", dest)
        };
        res.render(Text::Plain(info));
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("file not found in request"));
    };
}
