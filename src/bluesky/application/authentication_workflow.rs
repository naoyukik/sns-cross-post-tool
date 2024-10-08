use crate::bluesky::application::dto::access_token_dto::AccessToken;
use crate::bluesky::domain::login::login_repository::LoginRepository;
use crate::bluesky::infrastructure::login_repository_impl::LoginRepositoryImpl;

pub fn login() -> AccessToken {
    let login_result = LoginRepositoryImpl::login();
    match login_result {
        Ok(dto) => dto,
        Err(e) => panic!("Failed to login function: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::bluesky::application::authentication_workflow::login;
    use std::panic;

    #[test]
    #[should_panic(expected = "Failed to login function")]
    fn get_error_when_login_fails() {
        let result = panic::catch_unwind(|| login());
        assert!(result.is_err());
    }
}
