use rayon::prelude::*;

use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia; 


struct ProcessedPage {
    title: String,
    data: String,
}


const TITLES: [&str; 11] = [
    "Giannis Antetokounmpo",
    "James Harden",
    "Russell Westbrook",
    "Stephen Curry",
    "Kevin Durant",
    "LeBron James",
    "Kobe Bryant",
    "Michael Jordan",
    "Shaquille O'Neal",
    "Kanye West",
    "Elon Musk",
];


fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = match page.get_title() {
                Ok(title) => title,
                Err(_) => {
                    println!("Errorrrr for Title");
                        String::new()
                    }
    };

    let content = match page.get_content() {
        Ok(content) => content,
        Err(_) => {
            println!("Errorrrr for Title");
            String::from("")
        }
    };

    ProcessedPage{
        title,
        data: content,
    }
}


fn main() {
    let start = std::time::Instant::now();

    let wikipedia = Wikipedia::<Client>::default();

    let pages_from_titles: Vec<_> = TITLES
                                .par_iter()
                                .map(|&t| wikipedia.page_from_title(t.to_string()))
                                .collect();


    let processed_pages: Vec<_> = pages_from_titles
                                .par_iter()
                                .map(process_page)
                                .collect();


    for page in &processed_pages {
        let start_page_time = std::time::Instant::now();

        println!("Title: {}", page.title);

        let first_sentence: &str = match page.data.split('.').next() {
            Some(sentence) => {
                sentence
            },
            None => {
                println!("Page is empty");
                ""
            }    
        };
        
        println!("First sentence of {} is >>>>>>>>>>>>>>>>>>>>>>", page.title);
        print!("{}", first_sentence);


        let word_count: usize = page.data.split_whitespace().count();
        println!("Count is: {}", word_count);

        //prints time it took to process each page
        println!("Page Time: {:?}", start_page_time.elapsed());
    }


    //descriptive statistics of: total time, average time per page, and total number of pages, as well as the number of threads used
    println!("Total Time elapsed: {:?}", start.elapsed());

    println!("Avg Time per page: {:?}",
        start.elapsed() / TITLES.len() as u32
    );


    println!("Total no of pages: {}", &processed_pages.len());


    println!("Num of threads: {}", rayon::current_num_threads());
}