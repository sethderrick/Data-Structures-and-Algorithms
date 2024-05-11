use std::time::Instant;

fn find_nemo2(fish: &[&str]) {
    let start = Instant::now();

    for item in fish {
        if *item == "nemo" {
            println!("Found NEMO!");
        }
    }

    let duration = start.elapsed();
    println!(
        "Call to find Nemo took {} milliseconds.",
        duration.as_millis()
    );
}

fn main() {
    let fish = ["dory", "bruce", "marlin", "nemo"];
    let nemo = ["nemo"];
    let everyone = [
        "dory", "bruce", "marlin", "nemo", "gill", "bloat", "nigel", "squirt", "darla", "hank",
    ];
    let large = vec!["nemo"; 10000]; // Creating a large vector with 10,000 "nemo"

    find_nemo2(&everyone);
}
