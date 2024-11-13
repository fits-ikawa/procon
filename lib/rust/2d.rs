fn add_guard<T: Clone>(field: Vec<Vec<T>>, guard: T) -> Vec<Vec<T>> {
    assert!(!field.is_empty());
    assert!(!field[0].is_empty() && field.iter().map(|row| row.len()).all_equal());

    use std::iter::once;
    let width = field[0].len();

    once(vec![guard.clone(); width + 2])
        .chain(field.into_iter().map(|row| {
            once(guard.clone())
                .chain(row)
                .chain(once(guard.clone()))
                .collect()
        }))
        .chain(once(vec![guard.clone(); width + 2]))
        .collect()
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}

const IDIR4: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn idir4(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    IDIR4.iter().map(move |&(dx, dy)| (x + dx, y + dy))
}

const UDIR8: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

fn udir8(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR8.iter().map(move |&(dx, dy)| {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        (nx, ny)
    })
}

const IDIR8: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn idir8(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    IDIR8.iter().map(move |&(dx, dy)| (x + dx, y + dy))
}
