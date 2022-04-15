const COUPON_CODE: &str = "new2023";
const COUPON_DISCOUNT: f64 = 10.5;

fn main() {
    let total: f64;
    let has_discount: bool;

    let mut sub_total = 980.64;
    let coupon = "new2034".to_string();

    let discount = if coupon == COUPON_CODE {
        has_discount = true;
        COUPON_DISCOUNT
    } else {
        has_discount = false;
        0.0
    };

    let tax: f64 = {
        match sub_total as u64 {
            0..=1000 => 10.0,
            1001 => 11.0,
            n if n > 1000000 => 50.00,
            _ => 20.0,
        }
    };

    let (total, _, taxes) = {
        let discount_value = sub_total * (discount / 100.0);
        let tax_value = sub_total * (tax / 100f64);
        let result = sub_total - discount_value - tax_value;
        (result, discount_value, tax_value)
    };

    println!(
        "\nTax:\t\t{:.2}%\nTax amount:\t${3:.2}\nSub total:\t${1:.2}\nTotal:\t\t${2:.2}",
        tax, sub_total, total, taxes
    );
}
