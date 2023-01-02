pub struct LinearDophantine;

impl LinearDophantine {
    fn gcd(&self, a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }
        let mut x1 = 0;
        let mut y1 = 0;
        let d = self.gcd(b, a % b, &mut x1, &mut y1);
        *x = y1;
        *y = x1 - y1 * (a / b);
        return d;
    }

    pub fn find_any_solution(&self, a: i32, b: i32, c: i32, x: &mut i32, y: &mut i32) -> bool {
        let g = self.gcd(a.abs(), b.abs(), x, y);
        if c % g != 0 {
            return false;
        }

        *x *= c / g;
        *y *= c / g;
        if a < 0 {
            *x = -*x;
        }
        if b < 0 {
            *y = -*y;
        }

        /// update x and y to satisfy requirement here
        while *x < 0 || *y < 0 {
            if b > 0 && a < 0 {
                let temp = {
                    if *x < 0 {
                        ((*x as f32) / (b as f32)).ceil() as i32
                    } else {
                        ((*y as f32) / (a as f32)).ceil() as i32
                    }
                };
                *x += b * temp;
                *y -= a * temp;
            } else {
                let temp = {
                    if *x < 0 {
                        ((*x as f32) / (b as f32)).ceil() as i32
                    } else {
                        ((*y as f32) / (a as f32)).ceil() as i32
                    }
                };
                *x -= b * temp;
                *y += a * temp;
            }
        }
        return true;
    }
}
