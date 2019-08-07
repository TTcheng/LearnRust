use std::collections::HashMap;

// todo
//使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
//例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
//或者公司每个部门的所有员工按照字母顺排序的列表。
pub struct DepartmentEmployee {
    dep_emp_map: HashMap<String, Vec<String>>,
    all_emp_vec: Vec<String>,
}

impl DepartmentEmployee {
    pub fn new() -> DepartmentEmployee {
        let emp_vec: Vec<String> = Vec::new();
        let emp_map: HashMap<String, Vec<String>> = HashMap::new();
        DepartmentEmployee { dep_emp_map: emp_map, all_emp_vec: emp_vec }
    }
    fn add_employee_string(&mut self, department: String, employee: String) {
        let vec_string:Vec<String> = Vec::new();
        let dep_emps = self.dep_emp_map.entry(department).or_insert(vec_string);
        dep_emps.push(employee.clone());
        self.all_emp_vec.push(employee);
    }
    pub fn add_employee(&mut self, department: &str, employee: &str) {
        self.add_employee_string(department.to_string(), employee.to_string())
    }
    pub fn get_employees(&self, department: String) -> Option<&Vec<String>> {
        self.dep_emp_map.get(department.as_str())
    }
    pub fn all_employees(&self) -> &Vec<String> {
        &self.all_emp_vec
    }
}

// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
// 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
pub fn pig_latin(s: String) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let first_char = s.chars().nth(0);
    let mut is_vowel = false;
    for ch in vowels {
        if ch == first_char.unwrap() {
            is_vowel = true;
            break;
        }
    }
    let mut res = s.clone();
    if !is_vowel {
        res = s[1..].to_string();
        res.push('-');
        res.push(first_char.unwrap());
        res.push_str("ay");
    } else {
        res.push_str("-hay");
    }
    res
}

// 计算给定数组的平均数average、中位数median、众数mode
pub fn calc_aggregation(mut vec: Vec<i32>) {
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut mode = vec[0];
    let mut max_time = 1;
    for i in &vec {
        sum += *i;
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
        if max_time < *cnt {
            max_time = *cnt;
            mode = *i;
        }
    }
    let len = vec.len();
    println!("Sum: {}", sum);
    println!("mode: {}", mode);
    let avg: f64 = sum as f64 / len as f64;
    println!("average: {}", avg);
    quick_sort(&mut vec, 0, len - 1);
    let median = vec[len / 2];
    println!("median: {}", median);
}

// 快速排序
fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && nums[r] >= nums[left] {
            r -= 1;
        }
        while l < r && nums[l] <= nums[left] {
            l += 1;
        }
        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort(nums, left, l - 1);
    }

    quick_sort(nums, r + 1, right);
}