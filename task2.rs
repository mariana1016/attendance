fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 3;
        println!("Tracker: {}", tracker);
    };

    update();
    update();
}

fn main (){
    track_changes();
}