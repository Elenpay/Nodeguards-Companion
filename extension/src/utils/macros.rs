macro_rules! with_error_msg {
    ($result:expr, $error_set:stmt) => {
        match $result {
            Ok(_) => {},
            Err(_) => { 
                $error_set
                return;
            }
        }
    };
}

pub(crate) use with_error_msg;