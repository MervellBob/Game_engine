use std::io::stdin;

pub mod events {
    use log::info;

    pub fn add(event: fn() -> i64) {
        let mut events = ::events::ALL_EVENTS.lock().unwrap();
        events.push(event);

        let name = std::any::type_name_of_val(&event);
        info!("[DEFAULT] {name} event added");
        println!("Yes");
    }
}

fn test() -> i64 {
    println!("Allez");
    0
}

fn stdin_fun() -> i64 {
    let mut s=String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    println!("Input: {}",s);
    0
}

fn main() {
    env_logger::init();

    let stdin_var = stdin_fun as fn() -> i64;
    events::add(test);
    events::add(stdin_var);
    game_engine::entry();

    game_engine::remove(0);
    game_engine::entry();
}
