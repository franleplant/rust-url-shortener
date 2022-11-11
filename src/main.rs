use harsh::Harsh;

// creat a new harshid
fn get_id() -> String {
    let harsh = Harsh::builder().salt("salt goes here!").build().unwrap();
    let random_vec = (0..1).map(|_i| rand::random::<u64>()).collect::<Vec<u64>>();
    harsh.encode(&random_vec)
}

fn main() {
    println!("Hello, world! {}", get_id());
    println!("Hello, world! {}", get_id());
    println!("Hello, world! {}", get_id());
}
