use std::collections::BinaryHeap;
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
    let vector : Vec<i32> = vec![1,2,3,4,5];
    let vector2 = min_elems(vector, 2);
    for elem in &vector2
    {
        println!("{}", elem);
    }
}