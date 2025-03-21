use std::collections::{HashMap, HashSet};

const SHAPE_LIMIT: usize = 6;
const SHAPES: [Shapes; 6] = [
    Shapes::Triangle,
    Shapes::Square,
    Shapes::Pentagon,
    Shapes::Hexagon,
    Shapes::Heptagon,
    Shapes::Octagon,
];
pub fn solve() -> i128 {
    let mut begin_to_end_map: HashMap<i128, Vec<(i128, &'static str)>> = HashMap::new();

    for shape in SHAPES[..SHAPE_LIMIT].iter() {
        for i in 1.. {
            let result = shape.calculate(i);

            if result < 1000 || (result % 100) < 10 {
                continue;
            } else if result >= 10000 {
                break;
            }

            begin_to_end_map
                .entry(result / 100)
                .or_default()
                .push((result % 100, shape.name()));
        }
    }

    let mut answer = vec![];

    for (begin, ends) in begin_to_end_map.iter() {
        for end in ends {
            let mut visited: HashSet<(i128, &'static str)> = HashSet::new();
            let current = vec![(*begin * 100 + end.0, end.1)];

            dfs(&current, &begin_to_end_map, &mut visited, &mut answer);
        }
    }

    answer[0].iter().map(|(num, _)| num).sum::<i128>()
}

fn dfs(
    current: &[(i128, &'static str)],
    begin_to_end_map: &HashMap<i128, Vec<(i128, &'static str)>>,
    visited: &mut HashSet<(i128, &'static str)>,
    answer: &mut Vec<Vec<(i128, &'static str)>>,
) {
    if current.first().unwrap().0 / 100 == current.last().unwrap().0 % 100
        && current.len() == SHAPE_LIMIT
    {
        answer.push(current.to_owned());
        return;
    }

    visited.insert(*current.last().unwrap());

    if !begin_to_end_map.contains_key(&(current.last().unwrap().0 % 100)) {
        return;
    }

    for end in begin_to_end_map
        .get(&(current.last().unwrap().0 % 100))
        .unwrap()
    {
        if current
            .iter()
            .map(|(_, shape_name)| shape_name)
            .collect::<Vec<_>>()
            .contains(&&end.1)
        {
            continue;
        }

        if SHAPES[SHAPE_LIMIT..]
            .iter()
            .filter(|x| x.name() == end.1)
            .count()
            > 0
        {
            continue;
        }

        let new_num = end.0 + (current.last().unwrap().0 % 100) * 100;
        if visited.contains(&(new_num, end.1)) {
            continue;
        }

        let mut new_current = current.to_owned();
        new_current.push((new_num, end.1));
        dfs(&new_current, begin_to_end_map, visited, answer);
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Shapes {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
}

trait ShapeNumber {
    fn name(&self) -> &'static str;
    fn calculate(&self, x: i128) -> i128;
}

impl ShapeNumber for Shapes {
    fn name(&self) -> &'static str {
        match self {
            Shapes::Triangle => "Triangle",
            Shapes::Square => "Square",
            Shapes::Pentagon => "Pentagon",
            Shapes::Hexagon => "Hexagon",
            Shapes::Heptagon => "Heptagon",
            Shapes::Octagon => "Octagon",
        }
    }

    fn calculate(&self, x: i128) -> i128 {
        match self {
            Shapes::Triangle => x * (x + 1) / 2,
            Shapes::Square => x * x,
            Shapes::Pentagon => x * (3 * x - 1) / 2,
            Shapes::Hexagon => x * (2 * x - 1),
            Shapes::Heptagon => x * (5 * x - 3) / 2,
            Shapes::Octagon => x * (3 * x - 2),
        }
    }
}
