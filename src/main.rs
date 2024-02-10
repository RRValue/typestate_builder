mod http_response;

use http_response::HttpResponseBuilder;

fn main() {
    HttpResponseBuilder::new().print()
        .status("status".to_string()).print()
        .add_header("header01".to_string()).print()
        .add_header("header02".to_string()).print()
        .header_done().print()
        .body("body".to_string())
        .send();
}
