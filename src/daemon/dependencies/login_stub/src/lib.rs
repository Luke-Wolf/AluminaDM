use daemon_lib::LoginHandler;
pub struct LoginStub {}

impl LoginHandler for LoginStub {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
