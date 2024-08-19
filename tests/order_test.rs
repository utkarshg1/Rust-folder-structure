use store::{Category, Customer, Order, Product};
mod helpers;

#[test]
fn test_total_bill_without_discount() {
    helpers::common_setup();
    let product = Product::new(1, String::from("Book"), 250.0, Category::Books);
    let customer = Customer::new(1, String::from("Utkarsh"), String::from("example@test.com"));
    let order = Order::new(1, product, customer, 3);
    assert_eq!(format!("{:.2}", order.total_bill()), "825.00");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, String::from("Book"), 250.0, Category::Books);
    let customer = Customer::new(1, String::from("Utkarsh"), String::from("example@test.com"));
    let order = Order::new(1, product, customer, 15);
    assert_eq!(format!("{:.2}", order.total_bill()), "3712.50");
}
