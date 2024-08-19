use store::{Category, Customer, Order, Product};
fn main() {
    let product = Product::new(1, String::from("Laptop"), 35000.0, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
    let order = Order::new(1, product, customer, 2);
    println!("Total cost of Order is INR : {} Only", order.total_bill());
}
