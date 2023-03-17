use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn remove_header_start_end(response_text: &str, start_text: &str, end_text: &str) -> String {
    let start_index = response_text.find(start_text);
    let end_index = response_text.find(end_text);

    if let (Some(si), Some(ei)) = (start_index, end_index) {
        let mut replaced_text = response_text.to_string();
        let eil = ei + end_text.len();
        replaced_text.replace_range(si..eil, "<!-- removed simplabs header by rust-rewriter -->");
        return replaced_text;
    }

    response_text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_header_for_mainmatter() {
        let response_text = "<html><head></head><body><!--START--><div id='simplabsheader'>simplabs is Mainmatter now!</div><!--END--></body></html>";
        let result = remove_header_start_end(response_text, "<!--START-->", "<!--END-->");
        assert_eq!(result, "<html><head></head><body><!-- removed simplabs header by rust-rewriter --></body></html>");
    }

    #[test]
    fn it_do_nothing_if_header_doesnotexist() {
        let response_text = "<html><head></head><body></body></html>";
        let result = remove_header_start_end(response_text, "<!--START-->", "<!--END-->");
        assert_eq!(result, "<html><head></head><body></body></html>");
    }
}
