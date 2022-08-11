/// See readme before changing anything!
export function get_tinymce_content(editor) {
    return tinymce.get(editor).getContent()
}
export function set_tinymce_content(editor, content) {
    tinymce.get(editor).setContent(content);
}
