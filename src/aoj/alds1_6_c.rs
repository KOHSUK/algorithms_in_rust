use std::io::stdin;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Card {
    suit: String,
    number: i32,
}

impl Card {
    fn new(suit: String, number: i32) -> Self {
      Card { suit, number }
    }
}

fn print_cards(cards: &[Card], indeces: &[usize]) {
    for i in indeces.iter() {
        let card = &cards[*i];
        println!("{} {}", card.suit, card.number);
    }
}

fn print_arr(arr: &[usize]) {
    let result = arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}

fn check_if_stable(arr1: &[Card], indeces: &[usize], arr2: &[Card], indeces2: &[usize]) -> bool {
     for (i, &idx) in indeces.iter().enumerate() {
         let idx2 = indeces2[i];
         if arr1[idx].suit != arr2[idx2].suit {
            return false;
         }
     }
     true
}

pub fn run() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();
    let mut cards = Vec::new();
    let mut indeces = Vec::new();
    let mut cards2 = Vec::new();
    let mut indeces2 = Vec::new();
    for i in 0..n {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let card = buf.split_whitespace().collect::<Vec<&str>>();
        let card = Card::new(card[0].to_string(), card[1].parse().unwrap());
        cards2.push(card.clone());
        cards.push(card);
        indeces.push(i as usize);
        indeces2.push(i as usize);
    }
    quick_sort(&mut cards, &mut indeces, 0, n-1);
    // Merge sort is a stable sort algorithm
    merge_sort(&mut cards2, &mut indeces2, 0, (n-1) as usize);
    let stable = check_if_stable(&cards, &indeces, &cards2, &indeces2);
    println!("{}", if stable {"Stable"} else {"Not stable"});
    print_cards(&cards, &indeces);
}

fn quick_sort(arr: &mut Vec<Card>, indeces: &mut Vec<usize>, low: i32, high: i32) {
    if low < high {
        let idx_pivot = partition(arr, indeces, low as i32, high as i32);

        quick_sort(arr, indeces, low, idx_pivot - 1);
        quick_sort(arr, indeces, idx_pivot + 1, high);
    }
}

fn partition(arr: &mut Vec<Card>, indeces: &mut Vec<usize>, low: i32, high: i32) -> i32 {
    let mut i = low - 1;
    let low = low as usize;
    let high = high as usize;
    let pivot = &arr[indeces[high]];

    for j in low..high {
        if arr[indeces[j]].number <= pivot.number {
            i+=1;
            indeces.swap(i as usize, j);
        }
    }
    indeces.swap((i + 1) as usize, high);
    i + 1
}

fn merge_sort(arr: &mut Vec<Card>, indeces: &mut Vec<usize>, left: usize, right: usize) {
    let mid = left + (right - left) / 2;

    if left < right {
        merge_sort(arr, indeces, left, mid);
        merge_sort(arr, indeces, mid+1, right);
        merge(arr, indeces, left, mid, right);
    }

}

fn merge(arr: &mut Vec<Card>, indeces: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid + 1;
    let mut new_indeces = Vec::new();
    while new_indeces.len() < right - left + 1 {
        let mut l_item = i32::MAX;
        let mut r_item = i32::MAX;

        if i <= mid {
            l_item = arr[indeces[i]].number;
        }

        if j <= right {
            r_item = arr[indeces[j]].number;
        }

        if l_item <= r_item {
            new_indeces.push(indeces[i]);
            i+=1;
        } else {
            new_indeces.push(indeces[j]);
            j+=1;
        }
    }

    let mut i = left;
    for new_idx in new_indeces.iter() {
        indeces[i] = *new_idx;
        i+=1;
    }
}