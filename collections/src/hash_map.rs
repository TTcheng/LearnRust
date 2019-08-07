use std::collections::HashMap;

pub fn team_score() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // 插入或更新
    scores.insert(String::from("Blue"), 25);
    // 没有值时才插入
    scores.entry(String::from("Blue")).or_insert(30);

    // error
    // 所有权转移blue，yellow已经被清除
    // let score_of_blue_team = scores.get(&blue);
    scores
}

pub fn print_map(map: HashMap<String, i32>) {
    for (k, v) in map {
        println!("{}:{}", k, v)
    }
}

pub fn count_word(text: String) {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}