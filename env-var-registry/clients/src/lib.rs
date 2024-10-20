pub fn add(left: usize, right: usize) -> usize {
    left + right
}

const FOO_BAR: env_vars::EnvVar = env_vars::EnvVar::new("PREFIX", "FOO_BAR");

inventory::submit! {
    FOO_BAR
}

pub struct Doer(usize);

impl Doer {
    pub fn new(y: usize) -> Self {
        Doer(y)
    }
    pub fn add(&self, x: usize) -> usize {
        // let y : usize = FOO_BAR.read().parse().unwrap();
        let y = 5;
        x + y + self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
