//
// A little extra curricular work, outside the rust book, to study 
// BTreeMaps, which seemed appropriate after HashMap. Deeper study based on:
// https://doc.rust-lang.org/std/collections/index.html
//
// The section describing when to use each collection is helpful.
// 

fn main() {
    // Playing with BTreeMap collections and the "entry" API
    // 
    use std::collections::btree_map::BTreeMap;

    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8), "are these equal?");

    println!("Number of occurrences of each character");
    for (char, count) in &count {
        println!("{}: {}", char, count);
    }

    // 
    // Tracking the inebriation of customers at a bar
    //

    println!("=======================");
    println!("      New Tests        ");
    println!("=======================");
    
    // A client at a bar. They have a blood alchohol level
    // adding the debug line to enable printing later
    //
    #[derive(Debug)]
    struct Person { blood_alchohol: f32 }

    // All the orders made to the bar, by client ID.
    // assume they ordered one drink per order :-)
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1, 2, 4];

    // Our clients we set up a BTreeMap for them, so we can 
    // quickly look up their level
    let mut blood_alchohol = BTreeMap::new();

    // Then for every order, we update the level of alchohol by some amount
    // Ideally we'd reduce the level as time passes, since inebriation fades.
    for id in orders {
        // if this is the first time we've seen this customer, initialize them
        // with no blood alchohol. Otherwise, just retrieve them.
        let person = blood_alchohol.entry(id).or_insert(Person { blood_alchohol: 0.0 });

        // reduce their ba level. it takes time to order a drink
        person.blood_alchohol *= 0.9;

        // check if they're sober enough to have another beer.
        if person.blood_alchohol > 0.3 {
            //Too drunk...for now.
            println!("Sorry {}, I have to cut you off", id);
        } else {
            // Have another!
            person.blood_alchohol += 0.1;
        }
    }

    println!("==Blood Alchohol Data==");

    for (key, val) in &blood_alchohol {
        println!("Btree Person: {}, and BAC Value: {:?}", key, val.blood_alchohol);
    }

}
