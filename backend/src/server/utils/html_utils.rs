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
        .replace("$NC:USEAUTH", use_auth)
        .replace("$INK:USEAUTH", use_auth)
        .replace("$NC:FIREBASE_CONFIG", firebase_config)
        .replace("$INK:FIREBASE_CONFIG", firebase_config)
        .replace("$NC:BASEPATH", basepath)
        .replace("$INK:BASEPATH", basepath)
}
