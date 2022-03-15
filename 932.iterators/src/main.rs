// Programming a Series of Items with Iterators
//

fn main() {
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    let v1_iter = v1.iter();    // To: "associated type" with the Iterator trait
    println!("{:?}", v1_iter);

    // Iterators are lazy, they don't do anything until they are called

    // Iterator adaptors are also lazy, but iterator consumers are not
    // Here the closure || creates a new iterator with 1 added to each
    // item from the vector, but does nothing. 
    // 
    // Then collect() consumes the iter output, into a collection type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); 

    assert_eq!(v2, vec![2, 3, 4]);

    // map() is cool, it takes a closure allowing you to reuse iteration
    // behavior, customizing some behavior to your needs

}
