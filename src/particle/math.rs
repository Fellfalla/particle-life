pub fn interpolation(x:f64, x1:f64, x2:f64, y1:f64, y2:f64) -> f64 {
    let y_range = y2 - y1; 
    let x_range = x2 - x1;
    let alpha = (x-x1) / x_range;

    alpha * y_range + y1
}

pub fn clipped_interpolation(x:f64, x1:f64, x2:f64, y1:f64, y2:f64) -> f64 {

    let mut y = interpolation(x, x1, x2, y1, y2);
    
    let lower = y1.min(y2);
    let upper = y1.max(y2);

    y = y.max(lower).min(upper);

    return y;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation() {
        // test positive ranges
        assert_eq!(interpolation(0.5, 0.0, 1.0, 0.0, 10.0), 5.0);
        assert_eq!(interpolation(0.25, 0.0, 1.0, 0.0, 10.0), 2.5);
        assert_eq!(interpolation(0.75, 0.0, 1.0, 0.0, 10.0), 7.5);

        // test negative ranges
        assert_eq!(interpolation(0.5, 1.0, 0.0, 0.0, 1.0), 0.5);
        assert_eq!(interpolation(-0.5, 1.0, 0.0, 0.0, 1.0), 1.5);
        assert_eq!(interpolation(1.5, 1.0, 0.0, 0.0, 1.0), -0.5);
    }

    #[test]
    fn test_clipped_interpolation() {
        assert_eq!(clipped_interpolation(2.0, 0.0, 1.0, 0.0, 10.0), 10.0);
        assert_eq!(clipped_interpolation(-1.0, 0.0, 1.0, 0.0, 10.0), 0.0);
        assert_eq!(clipped_interpolation(0.5, 1.0, 0.0, 0.0, 1.0), 0.5);
    }
}
