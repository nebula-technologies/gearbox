pub fn is_false(b: &bool) -> bool {
    !*b // Returns true if `b` is false, causing the field to be skipped
}

pub fn is_true(b: &bool) -> bool {
    *b // Returns true if `b` is true, causing the field to be serialized
}
