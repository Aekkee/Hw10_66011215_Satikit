fn main() {
    println!("Hello, world!");
}

fn vflip(text: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in text {
        result.insert(0, i.to_string())
    }
    result
}

fn hflip(text: &[String]) -> Vec<String> {
    let max = text.iter().map(|x| x.len()).max().unwrap_or(0);
    return text.iter().map(|x| {format!("{}{}"," ".repeat(max - x.len()),x.chars().rev().collect::<String>())}).collect();
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vflip(&data), ["<==", "#####", "<--"]);
    assert_eq!(hflip(&data), ["  --<", "#####", "  ==<"]);

    let data2 = ["a", "abcde", "abc", "abcdefg"].map(|v| v.to_string());
    assert_eq!(vflip(&data2), ["abcdefg", "abc", "abcde", "a"]);
    assert_eq!(hflip(&data2), ["      a", "  edcba", "    cba", "gfedcba"]);
}

fn vcat(t1: &[String], t2: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.extend_from_slice(t1);
    result.extend_from_slice(t2);
    result
}

#[allow(unused_assignments)]
fn hcat(t1: &[String], t2: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let maxlen = t1.len().max(t2.len());
    let max = t1.iter().map(|x| x.len()).max().unwrap_or(0).max(t2.iter().map(|x| x.len()).max().unwrap_or(0));
    for i in 0..maxlen {
        let mut a = String::new();
        let mut b = String::new();
        let mut r = String::new();
        if i < t1.len() {
            a = t1[i].to_string();
        } else {
            a = " ".repeat(max)
        }
        if i < t2.len() {
            b = t2[i].to_string();
            if i < t1.len() {
            r = " ".repeat(max - t2[i].len())
            }
        }
        result.push(format!("{}{}{}", a, r, b));
    }
    result
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
        vcat(&data, &data),
        ["<--", "#####", "<==", "<--", "#####", "<=="]
    );
    assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
    assert_eq!(
        hcat(&data[..2], &data),
        [
            "<--  <--",
            "##########",
            "     <=="
        ]
    );

    let data2 = ["a", "abcde", "abc", "abcdefg"].map(|v| v.to_string());
    assert_eq!(vcat(&data2, &data2), ["a", "abcde", "abc", "abcdefg", "a", "abcde", "abc", "abcdefg"]);
    assert_eq!(hcat(&data2, &data2[..2]), ["a      a", "abcde  abcde", "abc", "abcdefg"]);
    assert_eq!(hcat(&data2[..2], &data2), ["a      a", "abcde  abcde", "       abc", "       abcdefg"]);
}
