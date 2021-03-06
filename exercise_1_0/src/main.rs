#[derive(Debug, PartialEq)]
pub struct Time {
    ms: u64,
}

impl Time {
    fn new(ms: u64) -> Self {
        Time { ms }
    }
}

fn in_hours(t: Time) -> u64 {
    t.ms / 1000 / 60 / 60
}

fn greater(t1: Time, t2: Time) -> Time {
    Time { ms: std::cmp::max(t1.ms, t2.ms) }
}

// implement without cloning
fn greatest(v: Vec<Time>) -> Time {
    let mut max_ms = v[0].ms;
    for t in v {
        if max_ms < t.ms {
            max_ms = t.ms;
        }
    }
    Time { ms : max_ms }
}

// references
fn time_diff_in_ms(t1: &Time, t2: &Time) -> u64 {
    if t1.ms > t2.ms { t1.ms - t2.ms } else { t2.ms - t1.ms }
}

fn greatest_ref(v: &Vec<Time>) -> &Time {
    let mut max_t = &v[0];
    for t in v {
        if max_t.ms < t.ms {
            max_t = t;
        }
    }
    max_t
}

fn main() {}

mod test {
    use super::*;
    #[test]
    fn test_in_hours() {
        let t = Time {
            ms: 3600 * 1000 * 5,
        };
        assert_eq!(5, in_hours(t));
    }

    #[test]
    fn test_greater() {
        let t1 = Time::new(5);
        let t2 = Time::new(1);
        let res = greater(t1, t2);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_greatest() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest(v);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_time_diff() {
        let t1 = Time::new(1);
        let t2 = Time::new(10);
        assert_eq!(9, time_diff_in_ms(&t1, &t2));
        assert_eq!(9, time_diff_in_ms(&t2, &t1));
    }

    #[test]
    fn test_greatest_ref() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest_ref(&v);
        assert_eq!(5, res.ms);
    }
}
