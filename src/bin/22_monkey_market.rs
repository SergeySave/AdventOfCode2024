/*
--- Day 22: Monkey Market ---

As you're all teleported deep into the jungle, a monkey steals The
Historians' device! You'll need to get it back while The Historians are
looking for the Chief.

The monkey that stole the device seems willing to trade it, but only in
exchange for an absurd number of bananas. Your only option is to buy
bananas on the Monkey Exchange Market.

You aren't sure how the Monkey Exchange Market works, but one of The
Historians senses trouble and comes over to help. Apparently, they've been
studying these monkeys for a while and have deciphered their secrets.

Today, the Market is full of monkeys buying good hiding spots. Fortunately,
because of the time you recently spent in this jungle, you know lots of
good hiding spots you can sell! If you sell enough hiding spots, you should
be able to get enough bananas to buy the device back.

On the Market, the buyers seem to use random prices, but their prices are
actually only pseudorandom! If you know the secret of how they pick their
prices, you can wait for the perfect time to sell.

The part about secrets is literal, the Historian explains. Each buyer
produces a pseudorandom sequence of secret numbers where each secret is
derived from the previous.

In particular, each buyer's secret number evolves into the next secret
number in the sequence via the following process:

- Calculate the result of multiplying the secret number by 64. Then, mix
this result into the secret number. Finally, prune the secret number.
- Calculate the result of dividing the secret number by 32. Round the
result down to the nearest integer. Then, mix this result into the
secret number. Finally, prune the secret number.
- Calculate the result of multiplying the secret number by 2048. Then,
mix this result into the secret number. Finally, prune the secret
number.

Each step of the above process involves mixing and pruning:

- To mix a value into the secret number, calculate the bitwise XOR of
the given value and the secret number. Then, the secret number becomes
the result of that operation. (If the secret number is 42 and you were
to mix 15 into the secret number, the secret number would become 37.)
- To prune the secret number, calculate the value of the secret number
modulo 16777216. Then, the secret number becomes the result of that
operation. (If the secret number is 100000000 and you were to prune
the secret number, the secret number would become 16113920.)

After this process completes, the buyer is left with the next secret number
in the sequence. The buyer can repeat this process as many times as
necessary to produce more secret numbers.

So, if a buyer had a secret number of 123, that buyer's next ten secret numbers would be:

15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254

Each buyer uses their own secret number when choosing their price, so it's
important to be able to predict the sequence of secret numbers for each
buyer. Fortunately, the Historian's research has uncovered the initial
secret number of each buyer (your puzzle input). For example:

1
10
100
2024

This list describes the initial secret number of four different secret-
hiding-spot-buyers on the Monkey Exchange Market. If you can simulate
secret numbers from each buyer, you'll be able to predict all of their
future prices.

In a single day, buyers each have time to generate 2000 new secret numbers.
In this example, for each buyer, their initial secret number and the 2000th
new secret number they would generate are:

1: 8685429
10: 4700978
100: 15273692
2024: 8667524

Adding up the 2000th new secret number for each buyer produces 37327623.

For each buyer, simulate the creation of 2000 new secret numbers. What is
the sum of the 2000th secret number generated by each buyer?

--- Part Two ---

Of course, the secret numbers aren't the prices each buyer is offering!
That would be ridiculous. Instead, the prices the buyer offers are just the
ones digit of each of their secret numbers.

So, if a buyer starts with a secret number of 123, that buyer's first ten
prices would be:

3 (from 123)
0 (from 15887950)
6 (from 16495136)
5 (etc.)
4
4
6
4
4
2

This price is the number of bananas that buyer is offering in exchange for
your information about a new hiding spot. However, you still don't speak
monkey, so you can't negotiate with the buyers directly. The Historian
speaks a little, but not enough to negotiate; instead, he can ask another
monkey to negotiate on your behalf.

Unfortunately, the monkey only knows how to decide when to sell by looking
at the changes in price. Specifically, the monkey will only look for a
specific sequence of four consecutive changes in price, then immediately
sell when it sees that sequence.

So, if a buyer starts with a secret number of 123, that buyer's first ten
secret numbers, prices, and the associated changes would be:

     123: 3
15887950: 0 (-3)
16495136: 6 (6)
  527345: 5 (-1)
  704524: 4 (-1)
 1553684: 4 (0)
12683156: 6 (2)
11100544: 4 (-2)
12249484: 4 (0)
 7753432: 2 (-2)

Note that the first price has no associated change because there was no
previous price to compare it with.

In this short example, within just these first few prices, the highest
price will be 6, so it would be nice to give the monkey instructions that
would make it sell at that time. The first 6 occurs after only two changes,
so there's no way to instruct the monkey to sell then, but the second 6
occurs after the changes -1,-1,0,2. So, if you gave the monkey that
sequence of changes, it would wait until the first time it sees that
sequence and then immediately sell your hiding spot information at the
current price, winning you 6 bananas.

Each buyer only wants to buy one hiding spot, so after the hiding spot is
sold, the monkey will move on to the next buyer. If the monkey never hears
that sequence of price changes from a buyer, the monkey will never sell,
and will instead just move on to the next buyer.

Worse, you can only give the monkey a single sequence of four price changes
to look for. You can't change the sequence between buyers.

You're going to need as many bananas as possible, so you'll need to
determine which sequence of four price changes will cause the monkey to get
you the most bananas overall. Each buyer is going to generate 2000 secret
numbers after their initial secret number, so, for each buyer, you'll have
2000 price changes in which your sequence can occur.

Suppose the initial secret number of each buyer is:

1
2
3
2024

There are many sequences of four price changes you could tell the monkey,
but for these four buyers, the sequence that will get you the most bananas
is -2,1,-1,3. Using that sequence, the monkey will make the following
sales:

- For the buyer with an initial secret number of 1, changes -2,1,-1,3
first occur when the price is 7.
- For the buyer with initial secret 2, changes -2,1,-1,3 first occur
when the price is 7.
- For the buyer with initial secret 3, the change sequence -2,1,-1,3
does not occur in the first 2000 changes.
- For the buyer starting with 2024, changes -2,1,-1,3 first occur when
the price is 9.

So, by asking the monkey to sell the first time each buyer's prices go down
2, then up 1, then down 1, then up 3, you would get 23 (7 + 7 + 9)
bananas!

Figure out the best sequence to tell the monkey so that by looking for that
same sequence of changes in every buyer's future prices, you get the most
bananas in total. What is the most bananas you can get?
 */
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/22_monkey_market.txt").unwrap();
    println!("{}", get_2000th_secret_number_sum(&input_file));
    println!("{}", get_maximum_bananas(&input_file));
}

const fn mix(secret_number: u64, number: u64) -> u64 {
    secret_number ^ number
}

const fn prune(number: u64) -> u64 {
    number % 16777216
}

const fn iter_secret_number(mut secret_number: u64) -> u64 {
    // Follow the basic instructions from the problem
    secret_number = prune(mix(secret_number, secret_number * 64));
    secret_number = prune(mix(secret_number, secret_number / 32));
    secret_number = prune(mix(secret_number, secret_number * 2048));

    secret_number
}

fn get_2000th_secret_number_sum(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let initial_secret_number = line.parse::<u64>().unwrap();

            let mut secret_number = initial_secret_number;
            for _ in 0..2000 {
                secret_number = iter_secret_number(secret_number);
            }

            secret_number
        })
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ChangeSequence([i8; 4]);

fn get_maximum_bananas(input: &str) -> u64 {
    // This probably isn't the fastest solution - it takes my machine ~8s in debug build (though
    // only ~0.5s in a release build)
    // But it is a pretty simple way to check all possible change sequences
    let mut bananas_for_sequence: HashMap<ChangeSequence, u64> = HashMap::new();
    input.lines()
        .for_each(|line| {
            let initial_secret_number = line.parse::<u64>().unwrap();

            let mut seen_sequences = HashSet::new();

            let mut secret_number = initial_secret_number;
            let mut changes = VecDeque::with_capacity(4);
            changes.push_back(None);
            changes.push_back(None);
            changes.push_back(None);
            changes.push_back(None);
            for _ in 0..2000 {
                let old_price = (secret_number % 10) as i8;
                secret_number = iter_secret_number(secret_number);
                let new_price = (secret_number % 10) as i8;
                changes.pop_front();
                changes.push_back(Some(new_price - old_price));
                if changes.iter().all(|x| x.is_some()) {
                    let sequence = ChangeSequence(
                        [
                            changes[0].unwrap(),
                            changes[1].unwrap(),
                            changes[2].unwrap(),
                            changes[3].unwrap(),
                        ]
                    );
                    if !seen_sequences.contains(&sequence) {
                        seen_sequences.insert(sequence);
                        bananas_for_sequence.insert(sequence, bananas_for_sequence.get(&sequence).cloned().unwrap_or_default() + (new_price as u64));
                    }
                }
            }
        });

    bananas_for_sequence.values().max().cloned().unwrap()
}

#[test]
fn test_part1() {
    const SECRET_NUMBER_ITERS: &[u64] = &[
        15887950,
        16495136,
        527345,
        704524,
        1553684,
        12683156,
        11100544,
        12249484,
        7753432,
        5908254,
    ];
    let mut secret_number = 123;
    for i in 0..SECRET_NUMBER_ITERS.len() {
        let secret_input = secret_number;
        secret_number = iter_secret_number(secret_input);
        assert_eq!(
            SECRET_NUMBER_ITERS[i],
            secret_number
        );
    }

    assert_eq!(
        37327623,
        get_2000th_secret_number_sum(r"1
10
100
2024"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        23,
        get_maximum_bananas(r"1
2
3
2024"
        )
    );
}