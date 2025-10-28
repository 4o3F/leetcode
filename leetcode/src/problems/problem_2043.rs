struct Bank(Vec<i64>);
impl Bank {
    fn new(b: Vec<i64>) -> Self {
        Self(b)
    }
    fn transfer(&mut self, a1: i32, a2: i32, m: i64) -> bool {
        a2 as usize <= self.0.len() && self.withdraw(a1, m) && self.deposit(a2, m)
    }
    fn deposit(&mut self, a: i32, m: i64) -> bool {
        let a = a as usize - 1;
        a < self.0.len() && {
            self.0[a] += m;
            true
        }
    }
    fn withdraw(&mut self, a: i32, m: i64) -> bool {
        let a = a as usize - 1;
        a < self.0.len() && self.0[a] >= m && {
            self.0[a] -= m;
            true
        }
    }
}

pub fn run() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    bank.withdraw(3, 10);
    bank.transfer(5, 1, 20);
    bank.deposit(5, 20);
    bank.transfer(3, 4, 15);
    bank.withdraw(10, 50);
}
