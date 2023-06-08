fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    
    let a_is_some: bool = a.is_some();
    dbg!(a_is_some);

    let a_is_none: bool = a.is_none();
    dbg!(a_is_none);

    let a_mapped: Option<i32> = a.map(|num: i32| num + 1); // some이 있을 때만 map 적용
    dbg!(a_mapped);
    
    let a_filtered: Option<i32> = a.filter(|num: &i32| num ==  &1);
    dbg!(a_filtered);
    
    let a_or_else: Option<i32> = a.or_else(|| Some(5));
    dbg!(a_or_else);

    let unwrapped: i32 = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}