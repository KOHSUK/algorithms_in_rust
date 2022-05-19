use std::io;

pub fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut cards: Vec<Card> = (0..n).map(|_| {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let tmp: Vec<String> = buf.split_whitespace().map(|x| x.to_string()).collect();
        (tmp[0].chars().next().unwrap(), tmp[1].parse().unwrap())
    }).collect::<Vec<Card>>();
    quick_sort(&mut cards);
    println!();
    print_cards(&cards);
}

type Card = (char, usize);

fn quick_sort(cards: &mut[Card]) {
    if cards.len() >= 2 {
        let p = partition(cards);

        quick_sort(&mut cards[..p]);
        quick_sort(&mut cards[p+1..]);
    } 
}

fn partition(cards: &mut[Card]) -> usize {
    let mut i = 0;
    let pivot_idx = cards.len() - 1;
    let target = cards[pivot_idx];

    for j in 0..pivot_idx {
        if cards[j].1 <= target.1 {
            cards.swap(i, j);
            i+=1;
        }
    }
    cards.swap(i, pivot_idx);
    i
}

fn print_cards(cards: &[Card]) {
    for (suit, number) in cards {
        println!("{} {}", suit, number);
    }
}