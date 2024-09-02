const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "French hens",
    "calling birds",
    "gold rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn num_to_numeral(num: &u8) -> &str {
    match num {
        0 => "Zero",
        1 => "A",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        _ => "",
    }
}

fn num_to_ordinal(num: &u8) -> &str {
    match num {
        0 => "Zeroeth",
        1 => "First",
        2 => "Second",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixtg",
        7 => "Seventh",
        8 => "Eighth",
        9 => "Nineth",
        10 => "Tenth",
        11 => "Eleventh",
        12 => "Twelveth",
        _ => "",
    }
}

fn twelve_days_of_christmas(day: &u8) -> String {
    let mut lyrics = format!(
        "On the {} day of christmas, my true love sent to me\n",
        num_to_ordinal(day)
    );

    for i in 1..=*day {
        let next_verse = format!("{} {}\n", num_to_numeral(&i), GIFTS[(i - 1) as usize]);

        lyrics.push_str(&next_verse);
    }

    lyrics
}

fn main() {
    println!("{}", twelve_days_of_christmas(&12));
}
