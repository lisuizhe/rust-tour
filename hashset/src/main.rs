#![allow(clippy::just_underscores_and_digits)]

use std::collections::HashSet;

fn hashset() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("greeks: {:?}", greeks);

    greeks.insert("delta");
    println!("greeks: {:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega! hooray!");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("removed delta");
    }
    println!("greeks: {:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint
    println!(
        "is {:?} disjoints with {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_subset(&_6_10)
    );

    // union, intersection
    println!(
        "items in either {:?} or {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );
    println!(
        "items in both {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.intersection(&_6_10)
    );
}

fn main() {
    hashset();
}
