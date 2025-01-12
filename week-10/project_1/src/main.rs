struct Laptop {
    price:u32, quantity:u32
}

impl Laptop{
    fn total(&self)->u32 {
        self.price * self.quantity
    }
}

fn main() {
    let hp = Laptop {
        price:650000,
        quantity:3
    };

    let ibm = Laptop {
        price:755000,
        quantity:3
    };

    let tosh = Laptop {
        price:550000,
        quantity:3
    };

    let dell = Laptop {
        price:850000,
        quantity:3
    };


    let _hp = hp.total();
    let _ibm = ibm.total();
    let _tosh = tosh.total();
    let _dell = dell.total();

    let tt = _hp + _ibm + _tosh + _dell;

    println!("Your total is : {}", tt);

}
