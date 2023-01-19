use std::str::FromStr;

struct Monkey {
    pub starting_items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test_modulo: u32,
    true_monkey_id: usize,
    false_monkey_id: usize,
    item_inspection_count: u32,
    is_reversible_operation: bool,
}

impl Monkey {
    pub fn new(
        starting_items: Vec<u32>,
        operation: Box<dyn Fn(u32) -> u32>,
        test_modulo: u32,
        true_monkey_id: usize,
        false_monkey_id: usize,
        is_reversible_operation: bool,
    ) -> Monkey {
        Monkey {
            starting_items,
            operation,
            test_modulo,
            true_monkey_id,
            false_monkey_id,
            item_inspection_count: 0,
            is_reversible_operation,
        }
    }

    pub fn add_item(&mut self, new_item: u32) {
        self.starting_items.push(new_item)
    }

    pub fn inspect_item(&mut self) -> (usize, u32) {
        let first_item = self.starting_items[0].clone();

        let worry_level = (self.operation)(first_item);

        let destination_monkey = if worry_level % self.test_modulo == 0 {
            self.true_monkey_id.clone()
        } else {
            self.false_monkey_id.clone()
        };

        self.starting_items.remove(0);

        self.item_inspection_count += 1;

        (destination_monkey, worry_level)
    }

    pub fn item_count(&self) -> usize {
        self.starting_items.len()
    }

    pub fn get_inspection_count(&self) -> &u32 {
        &self.item_inspection_count
    }
}

pub fn t_from_last_char<T>(input: &str) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
{
    input.split_whitespace().last().unwrap().parse::<T>()
}

pub fn task() {
    let mut monkeys: Vec<_> = include_str!("../../inputs/day_11/example.txt")
        .split("\n\n")
        .map(|monkey_lines| {
            let monkey_line: Vec<&str> = monkey_lines.split("\n").collect();

            let new_items = monkey_line[1]
                .split(&[':', ','][..])
                .into_iter()
                .filter_map(|f| f.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();

            let test_modulo = t_from_last_char::<u32>(monkey_line[3]).unwrap();
            let true_monkey_id = t_from_last_char::<usize>(monkey_line[4]).unwrap();
            let false_monkey_id = t_from_last_char::<usize>(monkey_line[5]).unwrap();

            let operation_operator_result = monkey_line[2].split_whitespace().nth(4).unwrap();

            let mut is_reversible = true;

            let operation_item: Box<dyn Fn(u32) -> u32> =
                match t_from_last_char::<u32>(monkey_line[2]) {
                    Ok(item) => match operation_operator_result {
                        "+" => {
                            is_reversible = false;
                            Box::new(move |f| f + item)
                        }
                        "*" => Box::new(move |f| f * item),
                        _ => panic!("symbol not parse-able"),
                    },
                    Err(..) => Box::new(|f| f * f),
                };

            Monkey::new(
                new_items,
                operation_item,
                test_modulo,
                true_monkey_id,
                false_monkey_id,
                is_reversible,
            )
        })
        .collect();

    for i in 0..20 {
        println!("Round: {:?}", i + 1);
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].item_count() {
                let current_monkey = &mut monkeys[monkey_index];

                let (next_monkey_index, next_item) = current_monkey.inspect_item();

                let next_monkey = &mut monkeys[next_monkey_index];

                next_monkey.add_item(next_item);
            }
        }
        for (j, m) in monkeys.iter().enumerate() {
            println!("idx {:?}, items --> {:?}", j, m.starting_items);
        }
        let inspection_counts = monkeys
            .iter()
            .map(|m| m.get_inspection_count())
            .collect::<Vec<&u32>>();

        println!("inspections {:?}", inspection_counts);
    }
}
