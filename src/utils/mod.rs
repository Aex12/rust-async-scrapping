pub fn convert_to_number(s: String) -> u32 {
    let number = s.replace(&[',', '€', ' '][..], "");
    number.parse::<u32>().unwrap()
}
