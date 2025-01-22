use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(n: usize) -> Vec<String> {
    let mut res = vec![
        String::from("Apple"),
        String::from("Banana"),
        String::from("Pear"),
        String::from("Luu"),
        String::from("hehh"),
        String::from("Roach"),
        String::from("Stew"),
    ];

    let mut rng = thread_rng();

    res.shuffle(&mut rng);

    res.into_iter().take(n).collect()
}