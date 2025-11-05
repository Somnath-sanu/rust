// String vs slices

fn main() {
    // write a fn that takes strings and return 1st word of it.
    let mut name = String::from("Hello word");
    let ans = first_word(&name);
    println!("{}", ans);

    name.replace_range(5..name.len(), "");

    println!("{}", name);

    // borrow but not completely
    // word is immutable reference of name
    let word = &name[0..5]; // last one is exclusive

    // name.clear(); // error ( owenership rule )

    /* you can have multiple immutable referece but only one mutable referece
    After that no nutable/immutable referece */

    println!("{}", word); // if we could clear name above then word would have pointed to something that doesn't exist.


    name.clear();

    // Slices can also be applied to other collections like vectors/arrays

    let arr = vec![1,2,3];
    let arr_slice = &arr[0..arr.len()-1];
    println!("Arr slice {:?}", arr_slice);

}

fn first_word(str: &String) -> &str {
    // let mut res: String = String::from("");
    let mut index = 0;

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        // res.push_str(&i.to_string());
        index = index + 1;
    }

    // return res;
    return &str[0..index];
    
}




fn demo(s: &String) -> &str {
  return &s[0..3];
  /*cannot return value referencing function parameter `s`
returns a value referencing data owned by the current function */
// when this fuc executes the varibles s removed as data will clear from heap so how can u return the referece of something that eventually will destroy

// so u have to pass referece as a parameter demo(s: &String)
}