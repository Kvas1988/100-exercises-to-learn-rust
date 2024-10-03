// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    fn val_name_is_empty(name: &String) {
        if name.is_empty() {
            panic!("Product Name cannot be empty");
        }
    }

    fn val_name_len(name: &String) {
        if name.len() > 300 {
            panic!("Product Name cannot be longer than 300 bytes");
        }
    }

    fn val_quantity(quantity: &u32) {
        if *quantity <= 0u32 {
            panic!("Quantity must be greater than zero");
        }
    }

    fn val_price(price: &u32) {
        if *price <= 0u32 {
            panic!("Price must be greater than zero");
        }
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Order::val_name_is_empty(&product_name);
        Order::val_name_len(&product_name);
        Order::val_quantity(&quantity);
        Order::val_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        self.unit_price * self.quantity
  }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn set_unit_price(&mut self, new_price: u32) {
        Order::val_price(&new_price);
        self.unit_price = new_price;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        Order::val_quantity(&new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_product_name(&mut self, new_name: String) {
        Order::val_name_len(&new_name);
        Order::val_name_is_empty(&new_name);
        self.product_name = new_name;
    }
}
