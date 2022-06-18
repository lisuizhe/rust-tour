fn match_statement() {
    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    match_statement();
}
