pub fn str_rsplit_delimiter(data: String, delimiter: &str) -> String {
    return data.rsplit(delimiter).collect::<Vec<&str>>()[0].to_string();
}

pub fn utf8_to_str(output: std::process::Output) -> String {
    return match output.status.success() {
        false => String::from(""),
        true => std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .replace(" ", "")
            .to_string(),
    };
}
