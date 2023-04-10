// Specify the WebAssembly Interface where your functions are defined
wai_bindgen_rust::export!("rustestest_lib.wai");
use crate::rustestest_lib::Programmer;

// Create your lib struct to be exported ALWAYS PUB
pub struct RustestestLib;

// Implement your lib trait in your lib Struct with Functions in it
impl rustestest_lib::RustestestLib for RustestestLib {
    // You must be aware that your functions have to follow the exact path that is defined in the .wai file
    fn reverse_listestest(input_list: Vec<u32>) -> Vec<u32> {
        let mut x = input_list;
        x.reverse();
        x
    }
    fn say_hi() -> String {
        String::from("Hello Lumiro!")
    }
    fn introduce_programmer(who: Programmer) -> String {
        let mut languages_string = String::new();
        
        for (index, language) in who.programming_languages.iter().enumerate(){
            languages_string.push_str(language);
            if index != who.programming_languages.len() - 1 {
                languages_string.push_str(", ");    
            } 
        }

        languages_string.push('.');

        String::from(
            format!(
                "This is: {}\nage: {}\nprogramming_languages: {}",
                who.name, who.age, languages_string
            ) 
        )
    }
    fn make_programmer(name: String, age: u32, programming_languages: Vec<String>) -> Programmer {
        Programmer { 
            name: name, 
            age: age, 
            programming_languages: programming_languages 
        }
    }
    fn high_computation(items: Vec<f32>) -> Vec<f32> {
        let mut items = items
            .iter()
            .map(|num| (num.powf(7.)) / 2.)
            .filter(|num| num > &10000.)
            .collect::<Vec<f32>>();

        items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        items
    }

}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_reverse_vector_values() {
        let reversed_vec = <RustestestLib as rustestest_lib::RustestestLib>::reverse_listestest(vec![1, 2, 3]);
        assert_eq!(reversed_vec, vec![3, 2, 1])
    }

    #[test]
    fn test_wrong_reversed_values() {
        let reversed_vec = <RustestestLib as rustestest_lib::RustestestLib>::reverse_listestest(vec![1, 2, 3]);
        assert_ne!(reversed_vec, vec![1, 2, 3])
    }

    #[test]
    fn test_say_hi() {
        let hi_message: &'static str = "Hello Lumiro!";
        assert_eq!(hi_message, <RustestestLib as rustestest_lib::RustestestLib>::say_hi())
    }
    

    #[test]
    fn test_make_programmer() {
        let programmer_name = "Jaw".to_string();
        let programmer_age = 19;
        let programing_languages = vec!["Python".to_string(), "Rust".to_string()]; 
        
        let new_programmer = <RustestestLib as rustestest_lib::RustestestLib>::make_programmer(
            programmer_name,
            programmer_age,
            programing_languages,
        );
        
        assert_eq!("Jaw".to_string(), new_programmer.name);
        assert_eq!(19, new_programmer.age);
        assert_eq!(vec!["Python".to_string(), "Rust".to_string()], new_programmer.programming_languages);
    }

    #[test]
    fn test_introduce_programmer() {
        let programmer_name = "Jaw".to_string();
        let programmer_age = 19;
        let programing_languages = vec!["Python".to_string(), "Rust".to_string()]; 
        
        let new_programmer = <RustestestLib as rustestest_lib::RustestestLib>::make_programmer(
            programmer_name,
            programmer_age,
            programing_languages,
        );

        assert_eq!(
            "This is: Jaw\nage: 19\nprogramming_languages: Python, Rust.".to_string(), 
            <RustestestLib as rustestest_lib::RustestestLib>::introduce_programmer(new_programmer)
        )
    }
    #[test]
    fn test_computate() {
        let reversed_vec = <RustestestLib as rustestest_lib::RustestestLib>::high_computation(vec![4., 6., 7.]);
        assert_eq!(reversed_vec, vec![139968.0, 411771.5])
    }
}