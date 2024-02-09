struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool
}

enum ProductCategory {
    Books,
    Clothing,
    Electrics,
}

fn main(){
    let category = ProductCategory::Electrics;
    let product = Product{
        name:String::from("TV"),
        category,
        price: 200,
        in_stock: true
    },
}