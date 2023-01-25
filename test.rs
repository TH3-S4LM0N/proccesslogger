use std::collections::HashSet;
fn main() {
    
    let s1: HashSet<String> = ["aa".to_string(), "bb".to_string()].iter().cloned().collect();
    let s2: HashSet<String> = ["aa".to_string(), "cc".to_string()].iter().cloned().collect();
    let a = &s1 - &s2;
    let b = &s2 - &s1;
    
    let union: HashSet<&String> = a.union(&b).collect();
    dbg!(union);

    /*
    let mut a = HashSet::new();
    a.insert("aa".to_string());
    a.insert("bb".to_string());
    let mut b = HashSet::new();
    b.insert("aa".to_string());
    b.insert("cc".to_string());

    let c = a - b;

    dbg!(c);

    
    unreachable!();
    let a = vec!["a".to_string(), "b".to_string()];
    let b = vec!["a".to_string(), "c".to_string()];

    // the end result should be
    //rv = (["b"], ["c"])

    let mut rv: (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let mut itera = 0;
    let mut iterb = 0;
    for itema in &a {
        println!("A:\nitem: {}\niters {}", itema, itera);
        for itemb in &b {
            println!("B:\nitem: {}\niters {}", itemb, iterb);
            if itema != itemb {
                
                
                //rv.0.push(format!("{itema}"));
                //rv.1.push(format!("{itemb}"));
            }
            iterb += 1;
        }
        itera += 1;
    }

    dbg!(rv);
    */
}