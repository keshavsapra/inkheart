use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};

pub fn get_pdf_disposition(filename: &String) -> ContentDisposition {
    let mut params = vec![];
    params.push(DispositionParam::Filename(filename.clone()));
    ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: params,
    }
}

pub fn get_image_disposition(filename: &String) -> ContentDisposition {
    let mut params = vec![];
    params.push(DispositionParam::Filename(filename.clone()));
    ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: params,
    }
}
