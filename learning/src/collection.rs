use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", even_filter(&vec)); // {:?} -> debug trait

    // even_filter_2(&mut vec) -> directly update vec
    even_filter_2(&mut vec);

    println!("{:?}", vec);

    vec.push(4);
    vec.push(5);
    vec.push(6);
    vec.push(7);

    even_filter_2(&mut vec);

    println!("{:?}", vec);

    // Another way to initialize a vector
    let mut v: Vec<i32> = vec![1, 2, 4];
    v.push(1);

    // Hashmap
    let mut users: HashMap<String, u16> = HashMap::new();
    users.insert(String::from("raman"), 18);
    users.insert(String::from("iishu"), 11);

    let age = users.get("raman");
    match age {
        Some(val) => println!("{}", val),
        None => println!("Error"),
    }

    // vectors of tuples
    let input_vec = vec![
        (String::from("shanu"), 20), // name -> age
        (String::from("rihan"), 19),
    ];

    let hm = group_values_by_keys(input_vec);
    println!("{:?}", hm);

    // Iterators
    // 1
    let mut arr = vec![1, 2, 3];
    let arr_itr = arr.iter_mut();
    for v in arr_itr {
        *v = *v + 1;
    }
    println!("{:?}", arr);

    // 2
    let arr_2 = vec![1, 2, 4];
    let mut arr_2_itr = arr_2.iter();

    while let Some(val) = arr_2_itr.next() {
        println!("{}", val);
    }

    // 3
    let arr_3 = vec![5, 6, 7];

    // let iter = arr_3.into_iter(); // this takes the ownerships

    // for val in iter {
    //   println!("{}", val);
    // }

    for val in arr_3 {
        // this takes the ownerships too
        println!("{}", val);
    }

    // println!("{:?}", arr_3); // error

    //* Write the logic to first filter all odd values then double each value and create a new vector */
    filter_and_map();


}

fn filter_and_map() {
    let arr = vec![1, 2, 3, 4, 5];
    let itr = arr.iter();

    let new_itr = itr.filter(|x| **x % 2 != 0).map(|x| x * 2);

    // use collect method to convert iterator back to the vector
    // we have to give here explict type
    let res: Vec<i32> = new_itr.collect();
    println!("{:?}", res);
}

fn group_values_by_keys(vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut even_vec: Vec<i32> = Vec::new(); // 
    for val in vec {
        if val % 2 == 0 {
            even_vec.push(*val);
        }
    }

    return even_vec;
}

// Approach 2
fn even_filter_2(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
