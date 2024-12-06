use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

type OrderPair = (usize, usize);
type OrderList = Vec<OrderPair>;
type PagesList = Vec<Vec<usize>>;

fn get_parts(contents: &str) -> anyhow::Result<(OrderList, PagesList)> {
    let (order, pages) = contents
        .split_once("\n\n")
        .ok_or(anyhow::anyhow!("Invalid input"))?;

    let order_parts: OrderList = order
        .split("\n")
        .flat_map(|x| {
            let (a, b) = x.split_once("|").ok_or(anyhow::anyhow!("Invalid input"))?;
            Ok::<(usize, usize), anyhow::Error>((a.parse()?, b.parse()?))
        })
        .collect();

    let pages_parts: PagesList = pages
        .split("\n")
        .map(|x| {
            let p: Vec<usize> = x
                .split(",")
                .flat_map(|x| Ok::<usize, anyhow::Error>(x.parse()?))
                .collect();
            p
        })
        .collect();

    Ok((order_parts, pages_parts))
}

struct Order {
    order: OrderList,
}

impl Order {
    fn new(order: OrderList) -> Self {
        Self { order }
    }

    fn compare(&self, x: usize, y: usize) -> Option<bool> {
        let ordered = self.order.contains(&(x, y));
        let reversed = self.order.contains(&(y, x));
        if ordered | reversed {
            Some(ordered)
        } else {
            None
        }
    }
}

fn pages_are_ordered(order: &Order, pages: &[usize]) -> bool {
    for first in 0..pages.len() {
        for second in first + 1..pages.len() {
            if let Some(ordered) = order.compare(pages[first], pages[second]) {
                if !ordered {
                    return false;
                }
            }
        }
    }
    true
}

fn order_pages(order: &Order, pages: &mut Vec<usize>) -> Vec<usize> {
    let mut ordered = Vec::with_capacity(pages.len());

    while !pages.is_empty() {
        let lowest = *pages
            .iter()
            .find(|x| {
                pages
                    .iter()
                    .all(|y| *x == y || order.compare(**x, *y).unwrap_or(true))
            })
            .unwrap();
        pages.remove(pages.iter().position(|x| *x == lowest).unwrap());
        ordered.push(lowest);
    }

    ordered
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let contents = read_to_string(input)?;
    let (order, pages_list) = get_parts(&contents)?;
    let mut count = 0;

    let order = Order::new(order);
    for pages in pages_list {
        if pages_are_ordered(&order, &pages) {
            println!("{:?} ordered", pages);
            count += pages[pages.len() / 2]
        }
    }
    Ok(count)
}

pub fn part_b(input: &Path) -> anyhow::Result<usize> {
    let contents = read_to_string(input)?;
    let (order, pages_list) = get_parts(&contents)?;
    let order = Order::new(order);
    let mut count = 0;

    for pages in pages_list {
        if !pages_are_ordered(&order, &pages) {
            let ordered = order_pages(&order, &mut pages.clone());
            count += ordered[ordered.len() / 2];
        }
    }
    Ok(count)
}
