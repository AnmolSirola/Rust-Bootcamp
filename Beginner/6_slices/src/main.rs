/* 

fn main() {
    // Slices are references to a contiguous sequence of
    // elements in a collection.

 let tweet = String::from(
    "This is my tweet and its long"
 );

 let trimmed_tweet: &str = &tweet[..20]; //string slice

 println!("{trimmed_tweet}");

}

*/
/* 

fn main() {

 let tweet = String::from(
    "This is my tweet and its long"
 );

 let trimmed_tweet: &str = trim_tweet(&tweet); 
 println!("{trimmed_tweet}");

}

fn trim_tweet(tweet: &String) -> &str{
    &tweet[..20]
}
*/


fn main() {

    let tweet = String::from(
        "This is my tweet and it's very very long"
    );
    
    let trimmed_tweet: &str = trim_tweet(&tweet);

    let tweet2 = "This is my tweet and it's very very long";
    let trimmed_tweet2 = trim_tweet(tweet2);

    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");

    let a = [1, 2, 3, 4, 5, 6]; // Slices also work with array
    let a_slice = &a[..3];   
    println!("{:?}", a_slice);
}

fn trim_tweet(tweet: &str /*&String*/) -> &str {
    &tweet[..20]
}

