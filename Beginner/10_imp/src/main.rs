/* 
struct Product{
    name: String,
    price: f32,
    in_stock: bool
}

impl Product{

    fn calculate_sales_tax(&self /* product: &Product*/ ) -> f32{
        self.price * 0.1  //product.price *0.1
    }
}

fn main() {
    let mut book = Product{
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
    };

    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
}
*/

/* 

struct Product{
    name: String,
    price: f32,
    in_stock: bool
}

impl Product{

    fn calculate_sales_tax(&self) -> f32{
        self.price * 0.1  
    }

    fn set_price(&mut self, price:f32) {
        self.price = price;
    }
}

fn main() {
    let mut book = Product{
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
    };

    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
}

*/



struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn new(name: String, price: f32) -> Product { //made for get_default_sales_tax fn. In rust its a common pattern for types to have associated fn called new which acts as a constructor. 
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }
    fn get_default_sales_tax() -> f32 { //associated funticons or static fn they are associated with a type however they dont work with the instance of that type. The difference between associated fn and a method is that associated fn dont take self as a parameter also  when calling a associated type we do not use the . syntax .  
        0.1
    }
    fn calculate_sales_tax(&self) -> f32 {  // immutable borrow to self. This form is used when we simplly want to reference the field of self without modifying anything
        self.price * Product::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) { //mutable borrow to self. We can mutablly borrow self
        self.price = price;
    }
    fn buy(self) -> i32 { // owned form of self. This is done when we want to transform one type to the another type. While also preventing the caller using the orginal instance.
        let name = self.name;
        println!("{name} was bought!");
        123
    }
}

fn main() {
    /*
    let mut book = Product{
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
    */
    
    let mut book = Product::new(
        String::from("Book"),
        30.0
    );
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
    book.set_price(1.0);
    book.buy();
}
