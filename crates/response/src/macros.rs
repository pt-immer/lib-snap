#[macro_export]
macro_rules! traced_guard {
    ($expr:expr $(,)?) => {{
        let __result = $expr;

        match __result {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("{}", err);

                return err.into();
            }
        }
    }};
}
