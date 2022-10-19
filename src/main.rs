use ordinal::Ordinal;

fn main() {
    for x in 1..=12 {
        let ordinal: String = Ordinal(x).to_string();

        println!("On the {ordinal} day of Christmas");

        println!("My good friends brought to me");

        if x > 11 {
            println!("All their good wishes");
        }

        if x > 10 {
            println!("Gifts for one and all");
        }

        if x > 9 {
            println!("Some mistletoe");
        }

        if x > 8 {
            println!("A guardian angel");
        }

        if x > 7 {
            println!("Gold and silver tinsel");
        }

        if x > 6 {
            println!("Candles a-glowing");
        }

        if x > 5 {
            println!("Little silver bells");
        }

        if x > 4 {
            println!("A shining star");
        }

        if x > 3 {
            println!("Four colored lights");
        }        

        if x > 2 {
            println!("Three boughs of holly");
        }

        let mut ending_verse: String = String::from("A song and a Christmas tree");

        if x > 1 {
            println!("Two candy canes");

            ending_verse = String::from("And ") + &ending_verse.replace("A", "a");
        }

        println!("{ending_verse} \n");
    }
}
