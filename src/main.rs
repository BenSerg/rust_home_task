use std::collections::BinaryHeap;

fn eq_pairs<T1, U1, T2, U2>(mut it1 : T1, mut it2 : T2) -> impl Iterator<Item = (U1, U2)> where
T1 : Iterator <Item = U1>,
U1 : PartialOrd<U2>,
T2 : Iterator<Item = U2>,
U2 : PartialOrd<U1>
{
    std::iter::from_fn(move || {
        let mut i= it1.next();
        let mut j = it2.next();
        while i.is_some() && j.is_some()
        {
            if i.as_ref().unwrap() < j.as_ref().unwrap()
            {
                i = it1.next();
            }
            else if i.as_ref().unwrap() > j.as_ref().unwrap()
            {
                j = it2.next();
            }
            else
            {
                return Some((i.unwrap(), j.unwrap()));
            }
        }
        None
    })
}
fn min_elems(x : Vec<i32>, n : i32) -> Vec<i32> 
{
    let mut heap = BinaryHeap::new();
    let mut vec : Vec<i32> = Vec::new();
    for elem in &x
    {
        if heap.is_empty()
        {
            heap.push(elem);
        }
        else
        {
            let max = match heap.peek()
            {
                Some(max) => max.clone(),
                None => todo!(),
            };
            if elem < max
            {
                if heap.len() == n as usize
                {
                    heap.pop();
                }
                heap.push(elem);
            }
            else if elem > max && heap.len() < n as usize
            {
                heap.push(elem);
            }
        }
    }
    let mut i = 0;
    for elem in &heap
    {
        if i < n
        {
            vec.push(**elem);
            i += 1;
        }
        else
        {
            break;
        }       
    }
    vec
}

fn main()
{
    //test for first task
    let vector : Vec<i32> = vec![1,2,3,4,5];
    let vector2 = min_elems(vector, 2);
    for elem in &vector2
    {
        println!("{}", elem);
    }
    let vector3 : Vec<i32> = vec![6,7,8,9];
    let vector4 : Vec<i32> = vec![6,8,10,12];
    let it = eq_pairs(vector3.iter(), vector4.iter());
    let vector5 : Vec<(&i32, &i32)> = it.collect();
    for item in &vector5
    {
     println!("{}", item.0);   
     println!("{}", item.1);   
    }
}