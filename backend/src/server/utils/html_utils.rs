pub fn escape_html_attribute(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn hydrate_index_html(
    html: &str,
    title: &str,
    img: &str,
    use_auth: &str,
    firebase_config: &str,
    basepath: &str,
) -> String {
    html.replace("$OGTITLE", title)
        .replace("$OGIMAGE", img)
        .replace("$INK:USEAUTH", use_auth)
        .replace("$INK:FIREBASE_CONFIG", firebase_config)
        .replace("$INK:BASEPATH", basepath)
}
