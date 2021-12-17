use std::{cmp::Ordering, ops::RangeInclusive};

fn main() {
    let target = Target {
        x: 117..=164,
        y: -140..=-89,
    };
    // Try to find an x_vel that lands in the target.
    // Lots between 15 and 117 work. Some don't. (Like, 100 doesn't work, but 117 does?)
    // Oh, I kinda get why.
    // for i in 75.. {
    //     let mut probe = Probe::origin_with_velocity(i, 0);
    //     let mut found = false;
    //     loop {
    //         probe.step();
    //         if target.x.contains(&probe.pos_x) {
    //             println!("x={} works", i);
    //             found = true;
    //             break;
    //         }
    //         if probe.vel_x == 0 {
    //             println!("x={} doesn't work", i);
    //             break;
    //         }
    //     }
    //     if found {
    //         break;
    //     }
    // }

    // Calculate the best launch. Actually, just run a bunch, and when it stops spitting out answers
    // try that one. Guess what: AoC accepted it. 🥳
    // let mut best_launch = 0;
    // for i in 0.. {
    //     let mut probe = Probe::origin_with_velocity(15, i);
    //     let mut max_height = 0;
    //     loop {
    //         probe.step();
    //         max_height = std::cmp::max(max_height, probe.pos_y);
    //         if target.contains(&probe) {
    //             if max_height > best_launch {
    //                 println!("15,{} works. Max height={}. BEST SO FAR!", i, max_height);
    //                 best_launch = max_height;
    //             } else {
    //                 println!("15,{} works. Max height={}", i, max_height);
    //             }
    //             break;
    //         }
    //         if probe.pos_y < -140 {
    //             break;
    //         }
    //     }
    // }
}

#[derive(Copy, Clone, Debug)]
pub struct Probe {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl Probe {
    pub fn origin_with_velocity(vel_x: i64, vel_y: i64) -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
        }
    }

    pub fn step(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        match self.vel_x.cmp(&0) {
            Ordering::Equal => {}
            Ordering::Less => {
                self.vel_x += 1;
            }
            Ordering::Greater => {
                self.vel_x -= 1;
            }
        }
        self.vel_y -= 1;
    }
}

struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl Target {
    fn contains(&self, probe: &Probe) -> bool {
        self.x.contains(&probe.pos_x) && self.y.contains(&probe.pos_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        let mut probe = Probe::origin_with_velocity(7, 2);
        probe.step();
        assert_eq!(probe.pos_x, 7);
        assert_eq!(probe.pos_y, 2);

        probe.step();
        assert_eq!(probe.pos_x, 13);
        assert_eq!(probe.pos_y, 3);

        probe.step();
        assert_eq!(probe.pos_x, 18);
        assert_eq!(probe.pos_y, 3);

        probe.step();
        assert_eq!(probe.pos_x, 22);
        assert_eq!(probe.pos_y, 2);

        probe.step();
        assert_eq!(probe.pos_x, 25);
        assert_eq!(probe.pos_y, 0);

        probe.step();
        assert_eq!(probe.pos_x, 27);
        assert_eq!(probe.pos_y, -3);

        probe.step();
        assert_eq!(probe.pos_x, 28);
        assert_eq!(probe.pos_y, -7);

        let mut probe = Probe::origin_with_velocity(6, 3);
        probe.step();
        assert_eq!(probe.pos_x, 6);
        assert_eq!(probe.pos_y, 3);

        probe.step();
        assert_eq!(probe.pos_x, 11);
        assert_eq!(probe.pos_y, 5);

        probe.step();
        assert_eq!(probe.pos_x, 15);
        assert_eq!(probe.pos_y, 6);

        probe.step();
        assert_eq!(probe.pos_x, 18);
        assert_eq!(probe.pos_y, 6);

        probe.step();
        assert_eq!(probe.pos_x, 20);
        assert_eq!(probe.pos_y, 5);

        probe.step();
        assert_eq!(probe.pos_x, 21);
        assert_eq!(probe.pos_y, 3);

        probe.step();
        assert_eq!(probe.pos_x, 21);
        assert_eq!(probe.pos_y, 0);

        probe.step();
        assert_eq!(probe.pos_x, 21);
        assert_eq!(probe.pos_y, -4);

        probe.step();
        assert_eq!(probe.pos_x, 21);
        assert_eq!(probe.pos_y, -9);
    }
}
