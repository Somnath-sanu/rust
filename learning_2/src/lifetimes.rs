// lifetimes

fn main() {
    let str1 = String::from("shanusssddsdsdsdddsdsd");
    let ans;
    {

        let str2 = String::from("Misrasdsdsds");
        ans = long(&str1, &str2);
        println!("{}",ans)
    }
    println!("{}",ans)
    // ans will be dangling pointer if str2 > str1


}

fn long<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return  s1;
    }
    return s2;
}