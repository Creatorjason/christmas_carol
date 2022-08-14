// A program to print the lyrics of "Twelve Days Of Christmas" a christmas carol song.
fn main(){
    let ordinal = ["", "first", "second", "third", 
                "fourth", "fifth", "sixth", "seventh", "eighth",
                 "ninth", "tenth", "eleventh", "twelfth"];
    let present = ["", "a partridge in a pear tree", "two turtle doves", "three French hens", 
                        "four colly birds", "five gold rings", "six geese a-laying",
                         "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", 
                        "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    for day in 1..13{
        println!("\nOn the {}, day of Christmas my true love gave to me...", ordinal[day]);
        for gift in (0..day).rev(){
            println!("- {}", present[gift]);
        }

    }


} 