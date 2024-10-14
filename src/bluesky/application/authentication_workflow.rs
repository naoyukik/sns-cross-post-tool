use crate::bluesky::application::dto::access_token_dto::AccessTokenDto;
use crate::bluesky::domain::login::login_repository::LoginRepository;
use crate::bluesky::infrastructure::login_repository_impl::LoginRepositoryImpl;

pub fn login() -> AccessTokenDto {
    let login_result = LoginRepositoryImpl::login();
    match login_result {
        Ok(dto) => AccessTokenDto::from(&dto),
        Err(e) => panic!("Failed to login function: {:?}", e),
    }
}

#[cfg(test)]
mod tests {}
