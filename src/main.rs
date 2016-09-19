extern crate expass;

fn main() {
    let database = expass::Database::new("expass.csv");
    let series = "0500";
    let number = 128883_u32;
    println!(
        "Exists {} {}: {}",
        series.to_string(),
        number,
        database.is_exist(
            series.to_string(),
            number,
        ),
    );
}
