use axum::{
  debug_handler,
  response::{Html, IntoResponse},
};

#[debug_handler]
pub async fn handle_playground() -> impl IntoResponse {
  let html = format!(
    r#"
    <!DOCTYPE html>
      <!doctype html>
      <html>
        <head>
          <title>{title}</title>
          <meta charset="utf-8" />
          <meta
            name="viewport"
            content="width=device-width, initial-scale=1" />
          <style>
            body {{
              margin: 0;
            }}
          </style>
        </head>
        <body>
          <script
            id="api-reference"></script>
          <script>
            var configuration = {{
              theme: 'deepSpace',
              customCss: `{scalar_css}`,
              spec: {{
                url: '{spec_url}'
              }}
            }}

            var apiReference = document.getElementById('api-reference')
            apiReference.dataset.configuration = JSON.stringify(configuration)
          </script>
          <script>{scalar_js}</script>
        </body>
      </html>
    "#,
    scalar_js = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/scalar.standalone.min.js")),
    scalar_css = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/rust-theme.css")),
    title = "MinaMesh Playground",
    spec_url = OPENAPI_SPEC
  );
  Html(html)
}

static OPENAPI_SPEC: &str =
  "https://raw.githubusercontent.com/coinbase/mesh-specifications/7f9f2f691f1ab1f7450e376d031e60d997dacbde/api.json";
