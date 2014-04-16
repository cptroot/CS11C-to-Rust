
#[deriving(Eq)]
enum Node {
    Cons(int, ~Node),
    Nil
}

/* Return a copy of a list. */
fn copy_list(list:&Node) -> ~Node {
    match list {
        &Cons(data, n) => ~Cons(data, copy_list(n)),
        &Nil => ~Nil
    }
}

/* Append two lists non-destructively.  The input lists are not altered. */
fn append_lists(list1:&Node, list2:&Node) -> ~Node {
    let mut copy1 = copy_list(list1);
    let copy2 = copy_list(list2);
    
    match (copy1, copy2) {
        (~Nil, copy2) => copy2,
        (copy1, ~Nil) => copy1,
        _ => {
            let mut iter:&Node = copy1;
            loop {
                let next = match iter {
                        &Cons(_, next) => &next,
                        &Nil => unreachable!()
                    };
                if *next == ~Nil {
                    *next = copy2;
                    break;
                }
                iter = &**next;
            }
            copy1
        }
    }
}

/* Make a reversed copy of a list.  The input list is not altered. */
fn reverse_list(list:&Node) -> ~Node {}

/* Print the elements of a list. */
fn print_list(list:&Node) {}

/* Return 1 if a list is sorted, otherwise 0. */
fn is_sorted(list:&Node) -> bool {}