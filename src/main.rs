use ordinal::Ordinal;

fn main() {
    //unique lyrics in order
    let lyrics: [&str; 12] = [
        "A song and a Christmas tree \n",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];

    for x in 1..=12 {
        let ordinal: String = Ordinal(x).to_string();

        println!("On the {ordinal} day of Christmas");

        println!("My good friends brought to me");

        for i in (1..=x).rev() {
            let mut verse: String = String::from(lyrics[i-1]);

            //refactor last passage so it becocomes "And a song and a Christmas tree"
            //instead of just "A song and a Christmas tree" starting with the second verse
            if (x > 1) & (verse == lyrics[0]) {
                verse = String::from("And ") + &verse.replace("A", "a");
            }

            println!("{verse}");
        }
    }
}
