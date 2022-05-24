// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod test_money_formatting {
    use moneta::{Dec, Decimal, Quantity, USD, UYW, VUV};

    #[test]
    fn test_unit_to_string() {
        assert_eq!(USD.to_string(), "USD");
    }

    #[test]
    fn test_unit_fmt() {
        assert_eq!(format!("{:^7}", UYW), "  UYW  ");
        assert_eq!(format!("{:>7.2}", VUV), "     VU");
    }

    #[test]
    fn test_money_to_string() {
        let val = Dec!(-18.95) * USD;
        assert_eq!(val.to_string(), "-18.95 USD");
        let val = Dec!(-18.95) * UYW;
        assert_eq!(val.to_string(), "-18.9500 UYW");
    }

    #[test]
    fn test_money_fmt() {
        let val = Dec!(2091.5) * USD;
        assert_eq!(format!("{:_>15}", val), "____2091.50 USD");
        assert_eq!(format!("{:<+15.0}", val), "+2092 USD      ");
        assert_eq!(format!("{:+15.4}", val), " +2091.5000 USD");
        let val = Dec!(295) * VUV;
        assert_eq!(format!("{:_>10}", val), "___295 VUV");
        assert_eq!(format!("{:_>10.1}", val), "_295.0 VUV");
        let val = Dec!(834.287) * UYW;
        assert_eq!(format!("{:+}", val), "+834.2870 UYW");
        assert_eq!(
            format!("{} {:+}", val.unit(), val.amount()),
            "UYW +834.287"
        );
        assert_eq!(
            format!("{} {:+12.4}", val.unit(), val.amount()),
            "UYW    +834.2870"
        );
    }
}

#[cfg(test)]
mod test_money_ops {
    use moneta::{Dec, Decimal, Quantity, USD, UYW};

    #[test]
    fn test_add_sub_same_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * USD;
        let z = x + y;
        assert_eq!(z.amount(), Dec!(63.29));
        assert_eq!(z.unit(), USD);
        assert_eq!(x + y, y + x);
        let z = x - y;
        assert_eq!(z.amount(), Dec!(-8.49));
        assert_eq!(z.unit(), USD);
    }

    #[test]
    #[should_panic]
    fn test_add_diff_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * UYW;
        let _z = x + y;
    }

    #[test]
    #[should_panic]
    fn test_sub_diff_currency() {
        let x = Dec!(27.4) * USD;
        let y = Dec!(35.89) * UYW;
        let _z = x - y;
    }

    #[test]
    fn test_mul_amnt_money() {
        let x = Dec!(7.5);
        let y = Dec!(23.85) * USD;
        let z = x * y;
        assert_eq!(z.amount(), Dec!(178.88));
        assert_eq!(z.unit(), USD);
        assert_eq!(x * y, y * x);
        let y = Dec!(23.85) * UYW;
        let z = x * y;
        assert_eq!(z.amount(), Dec!(178.875));
        assert_eq!(z.unit(), UYW);
    }

    #[test]
    fn test_div_money_amnt() {
        let x = Dec!(700.5) * USD;
        let y = Dec!(23.85);
        let z = x / y;
        assert_eq!(z.amount(), Dec!(29.37));
        assert_eq!(z.unit(), USD);
        let x = Dec!(700.5) * UYW;
        let z = x / y;
        assert_eq!(z.amount(), Dec!(29.3711));
        assert_eq!(z.unit(), UYW);
    }
}
