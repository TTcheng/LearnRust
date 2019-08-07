use crate::vectors::{use_vector, print_vec};
use crate::strings::create_hello;
use crate::strings::Greeter;
use crate::hash_map::{team_score, print_map, count_word};
use crate::problems::{calc_aggregation, pig_latin, DepartmentEmployee};

fn main() {
    let three_vec = use_vector();
    print_vec(three_vec.2);

    let greeter = Greeter { hello_i18n: create_hello(), name: "Jesse".to_string() };
    greeter.say_hello();

    let scores = team_score();
    print_map(scores);
    count_word("Hello world wonderful world".to_string());

    calc_aggregation(vec![1, 3, 5, 7, 2, 3, 5, 7, 5, 7, 7]);

    let apple = "apple".to_string();
    assert_eq!("apple-hay", pig_latin(apple));
    let first = "first".to_string();
    assert_eq!("irst-fay", pig_latin(first));

    let mut dep_emp = DepartmentEmployee::new();
    dep_emp.add_employee("IT", "Jesse");
    dep_emp.add_employee("IT", "Jack");
    dep_emp.add_employee("IT", "Monica");
    dep_emp.add_employee("HR", "Alice");
    dep_emp.add_employee("HR", "Olivie");

    let it_emps = dep_emp.get_employees("IT".to_string());
    let hr_emps = dep_emp.get_employees("HR".to_string());
    let all_emps = dep_emp.all_employees();
    println!("IT:{:?}", it_emps);
    println!("HR:{:?}", hr_emps);
    println!("ALL:{:?}", all_emps);
}

pub mod vectors;
pub mod strings;
pub mod hash_map;
pub mod problems;