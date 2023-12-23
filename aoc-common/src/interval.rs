#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Interval {
    start: isize,
    end: isize,
}

impl Interval {
    pub fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }

    pub fn split(&self, x: isize) -> Option<(Interval, Interval)> {
        if self.contains(x) {
            Some((Self::new(self.start, x), Self::new(x, self.end)))
        } else {
            None
        }
    }

    pub fn less_than(&self, x: isize) -> bool {
        self.end <= x
    }

    pub fn greater_than(&self, x: isize) -> bool {
        self.start > x
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, x: isize) -> bool {
        (self.start..self.end).contains(&x)
    }

    pub fn intersection(&self, other: &Interval) -> Option<Interval> {
        if self.end <= other.start || other.end <= self.start {
            None
        } else {
            Some(Interval::new(
                self.start.max(other.start),
                self.end.min(other.end),
            ))
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Intervals {
    intervals: Vec<Interval>,
}

impl Intervals {
    pub fn new(intervals: Vec<Interval>) -> Self {
        Self { intervals }
    }

    pub fn len(&self) -> usize {
        self.intervals.iter().map(|x| x.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn split(&self, x: isize) -> (Self, Self) {
        let index = self.intervals.partition_point(|i| i.end <= x);

        if index == self.intervals.len() {
            (self.clone(), Self::default())
        } else {
            let mut left = self.intervals[0..index].to_vec();
            let mut right = Vec::default();

            if let Some((a, b)) = self.intervals[index].split(x) {
                if !a.is_empty() {
                    left.push(a);
                }
                if !b.is_empty() {
                    right.push(b);
                }
            } else if x < self.intervals[index].start {
                right.push(self.intervals[index]);
            } else {
                left.push(self.intervals[index]);
            }

            right.extend(self.intervals[index + 1..].iter());
            (Self::new(left), Self::new(right))
        }
    }

    pub fn add(&mut self, interval: Interval) {
        if self.intervals.is_empty()
            || self.intervals[self.intervals.len() - 1].end < interval.start
        {
            self.intervals.push(interval);
            return;
        }

        let mut acc = Vec::default();
        let index = self.intervals.partition_point(|i| i.end < interval.start);
        acc.extend(self.intervals[0..index].iter());

        let cur = self.intervals[index];
        let (mut cur_s, mut cur_e) = if interval.end < cur.start {
            (interval.start, interval.end)
        } else {
            (interval.start.min(cur.start), interval.end.max(cur.end))
        };

        for i in self.intervals[index..].iter() {
            if i.start <= cur_e {
                cur_e = cur_e.max(i.end);
                continue;
            }

            acc.push(Interval::new(cur_s, cur_e));
            cur_s = i.start;
            cur_e = i.end;
        }

        acc.push(Interval::new(cur_s, cur_e));
        self.intervals = acc;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_split_test() {
        let interval = Interval::new(1, 10);
        assert_eq!(interval.split(0), None);
        assert_eq!(interval.split(10), None);
        assert_eq!(
            interval.split(1),
            Some((Interval::new(1, 1), Interval::new(1, 10)))
        );
        assert_eq!(
            interval.split(9),
            Some((Interval::new(1, 9), Interval::new(9, 10)))
        );
        assert_eq!(
            interval.split(7),
            Some((Interval::new(1, 7), Interval::new(7, 10)))
        );
    }

    #[test]
    fn intervals_split_test() {
        let intervals = Intervals::new(vec![
            Interval::new(-10, -5),
            Interval::new(-2, 5),
            Interval::new(7, 20),
        ]);

        assert_eq!(
            intervals.split(-11),
            (Intervals::default(), intervals.clone())
        );
        assert_eq!(
            intervals.split(-10),
            (Intervals::default(), intervals.clone())
        );
        assert_eq!(
            intervals.split(20),
            (intervals.clone(), Intervals::default())
        );
        assert_eq!(
            intervals.split(21),
            (intervals.clone(), Intervals::default())
        );
        assert_eq!(
            intervals.split(-9),
            (
                Intervals::new(vec![Interval::new(-10, -9)]),
                Intervals::new(vec![
                    Interval::new(-9, -5),
                    Interval::new(-2, 5),
                    Interval::new(7, 20)
                ])
            )
        );
        assert_eq!(
            intervals.split(-4),
            (
                Intervals::new(vec![Interval::new(-10, -5)]),
                Intervals::new(vec![Interval::new(-2, 5), Interval::new(7, 20)])
            )
        );
        assert_eq!(
            intervals.split(0),
            (
                Intervals::new(vec![Interval::new(-10, -5), Interval::new(-2, 0)]),
                Intervals::new(vec![Interval::new(0, 5), Interval::new(7, 20)])
            )
        );
        assert_eq!(
            intervals.split(10),
            (
                Intervals::new(vec![
                    Interval::new(-10, -5),
                    Interval::new(-2, 5),
                    Interval::new(7, 10)
                ]),
                Intervals::new(vec![Interval::new(10, 20),])
            )
        );
        assert_eq!(
            intervals.split(19),
            (
                Intervals::new(vec![
                    Interval::new(-10, -5),
                    Interval::new(-2, 5),
                    Interval::new(7, 19)
                ]),
                Intervals::new(vec![Interval::new(19, 20),])
            )
        );
    }

    #[test]
    fn intervals_add_test() {
        let mut intervals = Intervals::new(vec![Interval::new(0, 10), Interval::new(15, 20)]);

        intervals.add(Interval::new(-10, -5));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(0, 10),
                Interval::new(15, 20),
            ])
        );

        intervals.add(Interval::new(25, 30));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(0, 10),
                Interval::new(15, 20),
                Interval::new(25, 30),
            ])
        );

        intervals.add(Interval::new(12, 13));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(0, 10),
                Interval::new(12, 13),
                Interval::new(15, 20),
                Interval::new(25, 30),
            ])
        );

        intervals.add(Interval::new(9, 16));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(0, 20),
                Interval::new(25, 30),
            ])
        );

        intervals.add(Interval::new(-1, 1));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(-1, 20),
                Interval::new(25, 30),
            ])
        );

        intervals.add(Interval::new(30, 32));
        assert_eq!(
            intervals,
            Intervals::new(vec![
                Interval::new(-10, -5),
                Interval::new(-1, 20),
                Interval::new(25, 32),
            ])
        );
    }

    #[test]
    fn interval_intersection_test() {
        let interval = Interval::new(0, 10);
        assert_eq!(interval.intersection(&Interval::new(-5, 0)), None);
        assert_eq!(interval.intersection(&Interval::new(10, 15)), None);
        assert_eq!(
            interval.intersection(&Interval::new(-5, 5)),
            Some(Interval::new(0, 5))
        );
        assert_eq!(
            interval.intersection(&Interval::new(5, 15)),
            Some(Interval::new(5, 10))
        );
        assert_eq!(
            interval.intersection(&Interval::new(-5, 15)),
            Some(Interval::new(0, 10))
        );
        assert_eq!(
            interval.intersection(&Interval::new(5, 7)),
            Some(Interval::new(5, 7))
        );
    }
}
