pub mod fp8_i;
pub mod fp8_t;

#[cfg(test)]
mod tests {
    use super::fp8_i::E4M3;

    #[test]
    fn display_test() {
        let fp8i = E4M3::new(0x12);
        println!("{}", fp8i);
        let pmaxnormal = E4M3::new(0xFE); // 0x1111_1110
        let nmaxnormal = E4M3::new(0x7E); // 0x0111_1110
        let pminnormal = E4M3::new(0x88); // 0x1000_1000
        let nminnormal = E4M3::new(0x08); // 0x0000_1000
        println!("{}", pmaxnormal);
        println!("{}", nmaxnormal);
        println!("{}", pminnormal);
        println!("{}", nminnormal);
        let pmaxsubnormal = E4M3::new(0x87); // 0x1000_0111
        let nmaxsubnormal = E4M3::new(0x07); // 0x0000_0111
        let pminsubnormal = E4M3::new(0x81); // 0x1000_0001
        let nminsubnormal = E4M3::new(0x01); // 0x0000_0001
        println!("{}", pmaxsubnormal);
        println!("{}", nmaxsubnormal);
        println!("{}", pminsubnormal);
        println!("{}", nminsubnormal);
        let nan = E4M3::new(0xFF); // 0x1111_1111
        let neg_nan = E4M3::new(0x7F); // 0x1111_1111
        let zero = E4M3::new(0x80); // 0x1000_0000
        let neg_zero = E4M3::new(0x00); // 0x1000_0000
        println!("{}", nan);
        println!("{}", neg_nan);
        println!("{}", zero);
        println!("{}", neg_zero);
    }

    #[test]
    fn nan_test() {}
}
