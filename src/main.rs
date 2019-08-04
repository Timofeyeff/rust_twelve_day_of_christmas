use std::collections::HashMap;

fn main() {
    let numbers: HashMap<i32, &str> =
        [(1, "A"),
        (2,  "Two"),
        (3,  "Three"),
        (4,  "Four"),
        (5,  "Five"),
        (6,  "Six"),
        (7,  "Seven"),
        (8,  "Eight"),
        (9,  "Nine"),
        (10, "Ten"),
        (11, "Eleven"),
        (12, "Twelve")]
            .iter().cloned().collect();

    let days: HashMap<i32, &str> =
        [(1, "first"),
        (2,  "second"),
        (3, "thrid"),
        (4, "fourth"),
        (5, "fifth"),
        (6, "sixth"),
        (7, "seventh"),
        (8, "eighth"),
        (9, "ninth"),
        (10, "tenth"),
        (11, "eleventh"),
        (12, "tvelfth")]
            .iter().cloned().collect();

    let strings: HashMap<i32, &str> =
        [(1, "a partridge in pear tree"),
        (2,  "turtle doves"),
        (3, "French hens"),
        (4, "calling birds"),
        (5, "golden rings"),
        (6, "geese a-laying"),
        (7, "swans a-swimming"),
        (8, "maids a-milking"),
        (9, "ladies dancing"),
        (10, "lord a-leaping"),
        (11, "pipers piping"),
        (12, "drummers drumming")]
            .iter().cloned().collect();



    println!("Twelve Days of Christmas!");
    println!("");
    for i in 1i32..13 {
        match days.get(&i) {
            Some(&day) => println!("On the {} day of Christmas", day),
            _ => println!("No have day of Christmas"),
        }
        println!("my true love sent to me");
        let mut s_number = String::new();
        let mut s_string = String::new();
        for j in (1i32..i+1).rev() {
            match numbers.get(&j) {
                Some(&number) => s_number = number.to_string(),
                _ => s_number = "".to_string(),
            }
            match strings.get(&j) {
                Some(&string) => s_string = string.to_string(),
                _ => s_string = "".to_string(),
            }
            if j==1 {
                if i==1 {
                    println!("{} {}.", s_number, s_string);
                } else if i==12 {
                    println!("{}nd {}!", s_number, s_string);
                } else {
                    println!("{}nd {}.", s_number, s_string);
                }
            } else {
                println!("{} {},", s_number, s_string);
            }
        }
        println!("");
    }
    
}
