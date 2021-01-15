use notify_rust::Notification;
use std::{thread, time};

fn main() {
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
