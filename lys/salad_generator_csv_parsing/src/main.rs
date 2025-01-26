use salad_generator_csv_parsing::create_randomized_fruit_salad;

use clap::Parser;
use std::fs::read_to_string;


// Aim is to build a cli which takes either a CSV file or a string of fuits as input and returns this list. 

// Boiler Code required to run parser
#[derive(Parser)]
#[clap(
    version = "1.0.0",
    author = "HeliosX",
    about = "Custom Randomized Fruit Salad"
)]

// Create a struct which expects input to be either csv or string of fuits  
struct Opts {
    // Fruits input as strings, separated by ","

    csv: Option<String>,

    // Could hv easily made it just String to make life easier but who wants that?
    #[clap(short, long)]
    fruits: Option<String>,
}


// Converts the CSV file to to vector of Strings. trim() removes the leading and trailing whitespaces
fn csv_to_vec(csv: String) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}


fn display_fruits(fruit_salad: Vec<String>) {
    println!("Fruits are: ");

    for fruit in fruit_salad {
        print!("{} ", fruit)
    }
}


fn main() {
    // Create an instance of Opts struct
    let opts = Opts::parse();

    // Matching for csv attribute.
    if let Some(fruit_list) = opts.csv {
        // If found the file read it. If want to match for a case where no file or wrong file then manipulate this result<>
        let fruits = read_to_string(fruit_list)
                                        .expect("Could not read file");
        
        // Convert the CSV to Vector
        let mut fruit_salad = csv_to_vec(fruits);

        // Randomize the fruits
        fruit_salad = create_randomized_fruit_salad(fruit_salad);

        display_fruits(fruit_salad);
    } else {
        // For the case where fruits are manually typed
        if let Some(fruit_list)= opts.fruits {

            // Clean the input to convert to a vector of Strings(fruit)
            let mut fruit_salad = fruit_list.split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
            
            // Randomize
            fruit_salad = create_randomized_fruit_salad(fruit_salad);

            display_fruits(fruit_salad); 
        }
    }
}
