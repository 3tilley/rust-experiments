
pub struct EnvVar {
    pub env_var_prefix: &'static str,
    pub env_var_name: &'static str,
}

impl EnvVar {
    pub const fn new(env_var_prefix: &'static str, env_var_name: &'static str) -> Self {
        EnvVar {
            env_var_prefix,
            env_var_name,
        }
    }

    pub fn read(&self) -> String {
        let env_name = self.full_env_var_name();
        let val = std::env::var(env_name).unwrap();
        val
    }

    pub fn full_env_var_name(&self) -> String {
        format!("{}_{}", self.env_var_prefix, self.env_var_name)
    }
}

pub fn hello_string<'a>() -> &'a str {
    "Hello, World"
}

inventory::collect!(EnvVar);