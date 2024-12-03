use regex::Regex;

#[derive(Debug)]
enum Event {
    Start,
    Stop,
    Add(i64),
}

pub fn solve(input: String) {
    let mut events: Vec<(Event, usize)> = vec![];
    let mul_regex = Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#).unwrap();
    for cap in mul_regex.captures_iter(&input) {
        let pos = cap.get(0).unwrap().start();
        let num1 = cap.get(1).unwrap().as_str().parse::<i64>().expect("parse");
        let num2 = cap.get(2).unwrap().as_str().parse::<i64>().expect("parse");
        events.push((Event::Add(num1 * num2), pos));
    }
    let do_regex = Regex::new(r#"(do(n't)?\(\))"#).unwrap();
    for cap in do_regex.captures_iter(&input) {
        let pos = cap.get(0).unwrap().start();
        let event = match cap.get(1).unwrap().as_str() {
            "do()" => Event::Start,
            "don't()" => Event::Stop,
            _ => unreachable!(),
        };
        events.push((event, pos));
    }
    events.sort_unstable_by_key(|event| event.1);

    let mut on = 1;
    let mut ans = 0;
    for (event, _) in events {
        match event {
            Event::Stop => on = 0,
            Event::Start => on = 1,
            Event::Add(x) => ans += on * x,
        }
    }
    println!("{}", ans);
}
