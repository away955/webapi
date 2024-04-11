// use axum::{
//     extract::{DefaultBodyLimit, Multipart},
//     response::Html,
//     routing::{get, post},
//     Router,
// };
// use tower_http::limit::RequestBodyLimitLayer;

// pub fn api() -> Router {
//     Router::new()
//         .route("/upload/test", get(show_form))
//         .route("/upload", post(accept_form))
//         .layer(DefaultBodyLimit::disable())
//         .layer(RequestBodyLimitLayer::new(
//             250 * 1024 * 1024, /* 250mb */
//         ))
// }

// async fn show_form() -> Html<&'static str> {
//     Html(
//         r#"
//         <!doctype html>
//         <html>
//             <head></head>
//             <body>
//                 <form action="/upload" method="post" enctype="multipart/form-data">
//                     <label>
//                         Upload file:
//                         <input type="file" name="file" multiple>
//                     </label>

//                     <input type="submit" value="Upload files">
//                 </form>
//             </body>
//         </html>
//         "#,
//     )
// }

// async fn accept_form(mut multipart: Multipart) {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap().to_string();
//         let file_name = field.file_name().unwrap().to_string();
//         let content_type = field.content_type().unwrap().to_string();
//         let data = field.bytes().await.unwrap();

//         tracing::info!(
//             "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
//             data.len()
//         );
//     }
// }
