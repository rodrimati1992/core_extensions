use core_extensions::option_result_ext::ResultLikeExt;

use alloc::string::String;


#[test]
fn unwrap_unchecked_test(){
    unsafe{
        let ok: Result<String, ()> = Ok("foo".to_string());
        assert_eq!(ok.unwrap_unchecked_(), "foo");

        let err: Result<(), String> = Err("bar".to_string());
        assert_eq!(err.unwrap_err_unchecked_(), "bar");
    }
}