use sqlx::{postgres::PgQueryResult, Result};

pub fn ensure_affected(count: u64) -> impl FnOnce(PgQueryResult) -> Result<()> {
    move |pg_done| {
        if pg_done.rows_affected() == count {
            // log::info!("fffffffffffffffffffffff {:?}", pg_done.rows_affected());
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

pub mod actix_ext {
    use std::any;
    use std::ops::Deref;

    use actix_web::{dev::Payload, error::ErrorInternalServerError, FromRequest, HttpRequest};
    use futures::future;

    #[derive(Clone)]
    pub struct SafeData<T: Clone + 'static>(T);

    impl<T: Clone + 'static> SafeData<T> {
        /// Create new `SafeData` instance.
        pub fn new(state: T) -> Self {
            Self(state)
        }
    }

    impl<T: Clone + 'static> Deref for SafeData<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: Clone + 'static> FromRequest for SafeData<T> {
        type Config = ();
        type Error = actix_web::error::Error;
        type Future = future::Ready<Result<Self, actix_web::error::Error>>;

        #[inline]
        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            if let Some(st) = req.app_data::<SafeData<T>>() {
                future::ok(st.clone())
            } else {
                log::debug!(
                    "Failed to construct App-level Data extractor. \
                    Request path: {:?} (type: {})",
                    req.path(),
                    any::type_name::<T>(),
                );
                future::err(ErrorInternalServerError(
                    "App data is not configured, to configure use App::app_data()",
                ))
            }
        }
    }
}
