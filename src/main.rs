fn sort_usernames<T: AsRef<str>+Ord+PartialEq>(usernames: &mut Vec<T>){
    usernames.sort_by_cached_key(|s| s.as_ref().to_lowercase());
    // let mut lower_case_users: Vec<String> = Vec::new();
    // for user in usernames{
    //     lower_case_users.push(user.as_ref().to_lowercase());
    // }
    // println!("{:?}",lower_case_users);
    // lower_case_users.sort()

}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
