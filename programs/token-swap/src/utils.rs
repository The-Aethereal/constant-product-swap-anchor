pub fn integer_sqrt(x: u128) -> u128 {
    if x == 0 {
        return 0;
    }
    
    let mut left = 1u128;
    let mut right = x;
    let mut result = 0u128;
    
    while left <= right {
        let mid = (right + left) / 2;
        
        if mid <= x / mid { //without overflow
            result = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    result
}