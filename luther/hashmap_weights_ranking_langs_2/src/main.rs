use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn cal_wt(nums:&mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Finding thw exact count of years which is the difference
    for n in nums.values_mut() {
        *n = 2025 - *n;
    }

    // Finding MAX and MIN values for normalization ahead
    // min() -> Option() hence we need to unfold the values using pattern matching
    let min_year = match nums.values().min() {
        Some(n) => *n,
        None => 0,
    };

    let max_year = match nums.values().max() {
        Some(n) => *n,
        None => 0,
    };

    let mut res_wt = HashMap::new();

    for (lang, &year) in nums.iter() {
        // Here, there's a possibility when both min and max year are equal it can result in division by 0 error, Hence we need to tackle it
        let normalized_year = if max_year != min_year {
            (year - min_year) as f64 / (max_year - min_year) as f64
        } else {
            0.0
        };

        // Trash
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        
        res_wt.insert(String::from(lang), weight);
    }

    res_wt
}

fn main() {
    let mut map = init_languages();

    println!("{:?}", cal_wt(&mut map));
}
