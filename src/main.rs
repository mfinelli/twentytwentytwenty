use notify_rust::Notification;
use std::{env, thread, time};

const VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (&args[1] == "-v" || &args[1] == "--version") {
        println!("twentytwentytwenty version {}", VERSION);
    } else {
        let twenty_minutes = time::Duration::from_secs(1200);

        loop {
            thread::sleep(twenty_minutes);

            let mut notification = Notification::new()
                .summary("20 / 20 / 20")
                .body("Look at something 20 feet away for 20 seconds!")
                .show()
                .unwrap();

            for i in 1..=20 {
                thread::sleep(time::Duration::from_secs(1));

                notification.body(&format!(
                    "Look at something 20 feet away for \
                    20 seconds: {}",
                    20 - i
                ));
                notification.update();
            }
        }
    }
}
