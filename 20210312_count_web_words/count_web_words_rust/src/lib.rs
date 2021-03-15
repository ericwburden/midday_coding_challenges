extern crate hyper;
extern crate regex;

use std::str;
use std::collections::HashMap;
use hyper::Client;
use hyper::body::HttpBody as _;
use regex::{Regex, Error};


/// Uses the `hyper` crate to pull in the text of the site at the given URL and
/// provide a String representation of the HTML
#[tokio::main]
pub async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let uri = url.parse()?;
    let mut resp = client.get(uri).await?;

    let mut page_text = String::new();
    while let Some(chunk) = resp.body_mut().data().await {
        let c = chunk?;
        let s = match str::from_utf8(&c){
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        page_text.push_str(s);
    }

    Ok(page_text)
}


/// Given a string slice containing HTML text and the name of a target tag, extract
/// all the text between the opening and closing tag. I have not tested what happens
/// if there's more than one tag with the same name.
pub fn extract_from_tag(tag_name: &str, html: &str) -> Result<String, Error> {
    let re_str = format!("<{}[^>]*>(?P<{}>[\\s\\S]*)</{}>", tag_name, tag_name, tag_name);
    let re = Regex::new(&re_str)?;
    let contents = re.captures(&html).and_then(|cap| {
        cap.name(tag_name).map(|contents| contents.as_str())
    }).ok_or_else(|| Error::Syntax("Could not extract contents of tag.".to_string()))?;
    Ok(String::from(contents))
}


/// Strips all the HTML tags from an HTML text string
pub fn remove_tags(html: &str) -> String {
    let tag_re = Regex::new(r"<[^<>]*>").unwrap();
    let detagged = tag_re.replace_all(&html, " ");
    String::from(detagged)
}


/// Removes all characters other than alphanumerics, dashes, apostrophes, and 
/// whitespace from the string slice. Replaces all instances of multiple spaces
/// with a single space. Lowercases all words.
pub fn simplify_text(html: &str) -> String {
    let remove_re = Regex::new(r"[^a-zA-Z0-9-'\s]").unwrap();
    let cleaned_str = remove_re.replace_all(&html, " ");

    let multispace_re = Regex::new(r"\s+").unwrap();
    let cleaned_str = multispace_re.replace_all(&cleaned_str, " ");

    String::from(cleaned_str).to_lowercase()
}


/// Count the individual words in a space-separated string
pub fn count_words(html: &str) -> HashMap<&str, u8> {
    let mut word_counts = HashMap::new();
    for word in html.split_ascii_whitespace() {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    word_counts
}


/// Identifies the most common word or words (if there are ties) in a word count
pub fn most_common_words(word_counts: &HashMap<&str, u8>) -> Vec<(String, u8)> {
    let mut most_common = Vec::new();
    let max = word_counts.values().max().unwrap();
    for (key, value) in word_counts.iter() {
        if value == max {
            most_common.push((String::from(*key), *value));
        }
    }
    most_common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_my_work() {
        let html = fetch("http://paulgraham.com/simply.html").unwrap();
        
        let body_contents = extract_from_tag("body", &html).unwrap();
        let detagged_body = remove_tags(&body_contents);
        let simplified_text = simplify_text(&detagged_body);

        let word_counts = count_words(&simplified_text);
        let most_common = most_common_words(&word_counts);
        println!("The most common word(s) is/are {:?}", most_common);
    }
}
