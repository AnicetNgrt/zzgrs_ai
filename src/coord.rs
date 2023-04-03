#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl Coord {
    pub fn add_nowrap(&self, other: &Coord, w: i8, h: i8) -> Option<Coord> {
        let res = Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
        if res.x >= 0 && res.x < w && res.y >= 0 && res.y < h {
            Some(res)
        } else {
            None
        }
    }

    pub fn add_wrap(&self, other: &Coord, w: i8, h: i8) -> Coord {
        Coord {
            x: (self.x + other.x).rem_euclid(w),
            y: (self.y + other.y).rem_euclid(h),
        }
    }

    pub fn distance_wrap(&self, other: &Coord, w: i8, h: i8) -> i8 {
        let dist_nowrap = Self::distance(self, other);
        let dist_wrapy = if self.y > other.y {
            (self.x - other.x).abs() + (self.y - h - other.y).abs()
        } else if self.y < other.y {
            (self.x - other.x).abs() + (self.y - other.y - h).abs()
        } else {
            dist_nowrap
        };
        let dist_wrapx = if self.x > other.x {
            (self.x - w - other.x).abs() + (self.y - other.y).abs()
        } else if self.x < other.x {
            (self.x - other.x - w).abs() + (self.y - other.y).abs()
        } else {
            dist_nowrap
        };
        let dist_wrapboth = if self.x > other.x && self.y > other.y {
            (self.x - w - other.x).abs() + (self.y - h - other.y).abs()
        } else if self.x < other.x && self.y < other.y {
            (self.x - other.x - w).abs() + (self.y - other.y - h).abs()
        } else {
            dist_nowrap
        };
        dist_nowrap
            .min(dist_wrapx)
            .min(dist_wrapy)
            .min(dist_wrapboth)
    }

    pub fn distance_nowrap(&self, other: &Coord, w: i8, h: i8) -> i8 {
        let a = Coord {
            x: self.x.rem_euclid(w),
            y: self.y.rem_euclid(h),
        };
        let b = Coord {
            x: other.x.rem_euclid(w),
            y: other.y.rem_euclid(h),
        };
        Self::distance(&a, &b)
    }

    fn distance(a: &Coord, b: &Coord) -> i8 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }
}
