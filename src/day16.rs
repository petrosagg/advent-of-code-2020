use crate::lib::get_input_filter;
use std::collections::HashSet;

#[derive(Debug)]
enum Element {
    Rule(Vec<(u64, u64)>),
    Ticket(Vec<u64>),
    Mine(Vec<u64>),
}

pub fn first() {
    let elements: Vec<Element> = get_input_filter(16, 1, {
        enum Stage {
            Rules,
            Mine,
            Nearby,
        }
        use Stage::*;

        let mut cur_stage = Stage::Rules;

        move |l| {
            match l {
                "nearby tickets:" => {
                    cur_stage = Nearby;
                    return None;
                }
                "your ticket:" =>  {
                    cur_stage = Mine;
                    return None;
                }
                _ => (),
            }

            match cur_stage {
                Rules => {
                    let i = l.find(": ").unwrap();
                    let (_, s) = l.split_at(i + 2);
                    let ranges = s
                        .split(" or ")
                        .map(|r| {
                            let mut r = r.split('-').map(|n| n.parse::<u64>().unwrap());
                            let low = r.next().unwrap();
                            let high = r.next().unwrap();
                            (low, high)
                        })
                        .collect::<Vec<_>>();
                    Some(Element::Rule(ranges))
                },
                Nearby => {
                    let fields = l.split(',').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
                    Some(Element::Ticket(fields))
                }
                Mine => {
                    let fields = l.split(',').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
                    Some(Element::Mine(fields))
                }
                _ => None,
            }
        }
    });

    let mut rules = vec![];
    let mut tickets = vec![];
    let mut mine = vec![];

    for e in &elements {
        if let Element::Rule(a) = e {
            rules.push(a.clone());
        }
        if let Element::Ticket(a) = e {
            tickets.push(a.clone());
        }
        if let Element::Mine(a) = e {
            mine = a.clone();
        }
    }

    let mut valid_tickets = vec![];
    let mut error = 0;
    for ticket in &tickets {
        let mut valid = true;
        for field in ticket {
            if rules.iter().all(|r| r.iter().all(|r| *field < r.0 || *field > r.1)) {
                error += field;
                valid = false;
            }
        }
        if valid {
            valid_tickets.push(ticket.clone());
        }
    }
    dbg!(error);

    let num_fields = (&valid_tickets[0]).len();

    let mut remaining_rules = HashSet::new();
    remaining_rules.extend(rules.into_iter().enumerate());
    let mut remaining_fields = HashSet::new();
    remaining_fields.extend(0..num_fields);

    let mut guesses = vec![0; num_fields];
    while !remaining_fields.is_empty() {
        let mut fields_to_remove = vec![];

        for field in &remaining_fields {
            // find all possible rules that match all tickets at this particular field
            let guess = remaining_rules.iter().filter(|(i, rules)| {
                valid_tickets
                    .iter()
                    .map(|fields| fields[*field])
                    .all(|n| {
                        rules.iter().any(|r| r.0 <= n && n <= r.1)
                    })
            }).collect::<Vec<_>>();

            // If there is only one rules in our guess, that must be it
            if guess.len() == 1 {
                guesses[*field] = guess[0].0;
                remaining_rules.remove(&guess[0].clone());
                fields_to_remove.push(*field);
            }
        }

        for field in &fields_to_remove {
            remaining_fields.remove(field);
        }
    }


    let mut acc = 1;
    for (from, to) in guesses.iter().enumerate() {
        if *to < 6 {
            acc *= mine[from];
        }
    }
    dbg!(acc);
}




