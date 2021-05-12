pub fn min<T>(v1: T, v2: T) -> T
where T: Ord {
    return if v1 < v2 { v1 } else { v2 }
}
