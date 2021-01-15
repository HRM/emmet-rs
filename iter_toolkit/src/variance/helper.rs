pub fn rebuild_stack(n: usize, m: usize, indices: &mut Vec<usize>) -> Vec<Vec<usize>> {
    let mut stack = Vec::<Vec<usize>>::with_capacity(m);
    //reconstruct from last step
    stack.push((0..(n)).collect());
    if !indices.is_empty() {
        for i in 0..(indices.len() - 1) {
            if indices[i] < stack.last().unwrap().len() {
                let filtered = stack.last().unwrap()[indices[i]];
                stack.push(
                    stack
                        .last()
                        .unwrap()
                        .iter()
                        .filter(|e| **e != filtered)
                        .cloned()
                        .collect(),
                );
            } else {
                stack.push(
                    stack
                        .last()
                        .unwrap()
                        .clone(),
                );
            }
        }
        *(indices.last_mut().unwrap()) += 1_usize;
    } else {
        indices.push(0);
    }
    stack
}

pub fn push_stack(indices: &mut Vec<usize>, stack: &mut Vec<Vec<usize>>) {
    if stack.last().unwrap().len() > *indices.last().unwrap() {
        let filtered = stack.last().unwrap()[*(indices.last().unwrap())];
        stack.push(
            stack
                .last()
                .unwrap()
                .iter()
                .filter(|e| **e != filtered)
                .cloned()
                .collect()
        );
    }else{
        stack.push(
            stack
                .last()
                .unwrap()
                .clone()
        );
    }
    indices.push(0);
}

