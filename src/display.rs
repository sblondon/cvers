use std::cmp::Ordering;


pub fn display(order: Ordering)-> String{
    match order {
        Ordering::Less => "<".to_string(),
        Ordering::Equal => "=".to_string(),
        Ordering::Greater => ">".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_inf() {
        assert_eq!(display(Ordering::Less), "<".to_string());
    }
    #[test]
    fn test_display_equal() {
        assert_eq!(display(Ordering::Equal), "=".to_string());
    }
    #[test]
    fn test_display_sup() {
        assert_eq!(display(Ordering::Greater), ">".to_string());
    }
}
