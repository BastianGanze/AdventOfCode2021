#[derive(Clone)]
pub struct Grid<const N: usize, const M: usize> {
    fields: [[u8; N]; M],
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Grid<N, M> {
        Grid {
            fields: [[0; N]; M],
        }
    }

    pub fn dimensions(&self) -> (i16, i16) {
        (N as i16, M as i16)
    }

    pub(self) fn is_field_in_bounds(&self, y: i16, x: i16) -> bool {
        y >= 0 && x >= 0 && (y as usize) < N && (x as usize) < M
    }

    pub fn get_field(&self, y: i16, x: i16) -> Option<u8> {
        if !self.is_field_in_bounds(y, x) {
            return None;
        }

        Some(self.fields[y as usize][x as usize])
    }

    pub fn inc_field(&mut self, y: i16, x: i16) {
        if self.is_field_in_bounds(y, x) {
            self.fields[y as usize][x as usize] += 1;
        }
    }

    pub fn inc_field_if_not_0(&mut self, y: i16, x: i16) {
        if !self.is_field_in_bounds(y, x) {
            return;
        }

        if self.fields[y as usize][x as usize] == 0 {
            return;
        }

        self.fields[y as usize][x as usize] += 1;
    }

    pub fn set_field(&mut self, y: i16, x: i16, num: u8) {
        if self.is_field_in_bounds(y, x) {
            self.fields[y as usize][x as usize] = num;
        }
    }

    pub fn to_string(&self) -> String {
        let (a, b) = self.dimensions();
        let mut out: String = String::new();
        for y in 0..a {
            for x in 0..b {
                out.push_str(self.fields[y as usize][x as usize].to_string().as_str());
                out.push_str(" ");
            }
            out.push_str("\n");
        }
        out
    }
}
