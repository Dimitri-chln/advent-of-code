use super::direction::Direction;

pub trait SearchPattern<T> {
    fn search_pattern<const N: usize>(
        &self,
        pattern: [T; N],
        coordinates: (usize, usize),
        direction: Direction,
    ) -> bool;
}

impl<T> SearchPattern<T> for &[Vec<T>]
where
    T: PartialEq,
{
    fn search_pattern<const N: usize>(
        &self,
        pattern: [T; N],
        (i, j): (usize, usize),
        direction: Direction,
    ) -> bool {
        for k in 0..N {
            let Ok((i, j)) = step_direction((i, j), k, direction) else {
                return false;
            };

            if !self
                .get(i)
                .is_some_and(|row| row.get(j).is_some_and(|value| value == &pattern[k]))
            {
                return false;
            }
        }

        true
    }
}

fn step_direction(
    (i, j): (usize, usize),
    step: usize,
    direction: Direction,
) -> Result<(usize, usize), ()> {
    Ok(match direction {
        Direction::Up => (i.checked_sub(step).ok_or(())?, j),
        Direction::Down => (i.checked_add(step).ok_or(())?, j),
        Direction::Left => (i, j.checked_sub(step).ok_or(())?),
        Direction::Right => (i, j.checked_add(step).ok_or(())?),
        Direction::UpLeft => (
            i.checked_sub(step).ok_or(())?,
            j.checked_sub(step).ok_or(())?,
        ),
        Direction::UpRight => (
            i.checked_sub(step).ok_or(())?,
            j.checked_add(step).ok_or(())?,
        ),
        Direction::DownLeft => (
            i.checked_add(step).ok_or(())?,
            j.checked_sub(step).ok_or(())?,
        ),
        Direction::DownRight => (
            i.checked_add(step).ok_or(())?,
            j.checked_add(step).ok_or(())?,
        ),
    })
}
