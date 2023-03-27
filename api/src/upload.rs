use async_std::path::Path;
use salvo::{
    handler,
    prelude::StatusCode,
    writer::Text,
    {Request, Response},
};

#[handler]

pub async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;
    println!("{:#?}", file);
    if let Some(file) = file {
        let dest = format!("temp/{}", file.name().unwrap_or("file"));
        println!("{}", dest);
        let info = if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            format!("file not found in request efzdfdfdfd: {}", e)
        } else {
            format!("File uploaded to {}", dest)
        };
        res.render(Text::Plain(info));
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("file not found in request brewwwww"));
    };
}
