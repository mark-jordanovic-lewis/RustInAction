const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;
    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, exponent_, mantissa) = decode(sign, exponent,  fraction);
    let n_ = from_parts(sign_, exponent_, mantissa);

    println!("{} -> {}", n, n_);
    println!("field\t | as_bits\t\t\t| as_real");
    println!("sign\t | {:01b}\t\t\t\t| {}", sign, sign_);
    println!("exponent | {:08b}\t\t\t| {}", exponent, exponent_);
    println!("mantissa | {:023b}\t| {}", fraction, mantissa);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;
    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let sign = (-1.0_f32).powf(sign as f32);
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    let mut mantissa = 1.0_f32;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;

        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (sign, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}