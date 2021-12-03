use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DbErr;
use serde_json::json;

pub(crate) type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error("database error")]
    DbErr(#[from] DbErr),
    #[error("already exists")]
    Conflict,
    #[error("not found")]
    NotFount,
    #[error("{0:?}")]
    Error(#[from] eyre::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::http::Response<axum::body::BoxBody> {
        // special case unique constraint failure and return 409
        if let AppError::DbErr(DbErr::Exec(ref msg)) = self {
            // this is a ugly hack, but i'm not sure how to clean this up :(
            if msg.contains("UNIQUE constraint failed") {
                return AppError::Conflict.into_response();
            }
        };

        let message = format!("{:?}", self);
        let status = match self {
            AppError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::NotFount => StatusCode::NOT_FOUND,
            AppError::Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "status": "error", "message": message }));

        (status, body).into_response()
    }
}

/// Takes a list of handler expressions and `or`s them together
/// in a balanced tree. That is, instead of `a.or(b).or(c).or(d)`,
/// it produces `(a.or(b)).or(c.or(d))`, thus nesting the types
/// less deeply, which provides improvements in compile time.
///
/// It also applies `::warp::Filter::boxed` to each handler expression
/// when in `debug_assertions` mode, improving compile time further.
//
// The basic list splitting algorithm here is based on this gist:
// https://gist.github.com/durka/9fc479de2555225a787f
// It uses a counter from which two items are removed each time,
// stopping when the counter reaches 0. At each step, one item
// is moved from the left to the right, and thus at the end,
// there will be the same number of items in each list.
//
// The flow is as follows:
// - If there is one handler expression, debug_box it and return.
// - If there is more than one handler expression:
//   - First, copy the list into two: the one that will go into the
//     right side of the `or`, and one that will serve as a counter.
//     Recurse with these separated by semicolons, plus an empty `left`
//     list before the first semicolon.
//   - Then, as long as there are at least two items in the counter
//     list, remove them and move the first item on the right side of
//     the first semicolon (`head`) to the left side of the first semicolon.
//   - Finally, when there are one or zero items left in the counter,
//     move one last item to the left, make the call this macro on both the
//     left and right sides, and `or` the two sides together.
//
// For example, balanced_or_tree!(a, b, c, d, e) would take the following steps:
//
// - balanced_or_tree!(a, b, c, d, e)
// - balanced_or_tree!(@internal ; a, b, c, d, e ; a, b, c, d, e) // initialise lists
// - balanced_or_tree!(@internal a ; b, c, d, e ; c, d, e) // move one elem; remove two
// - balanced_or_tree!(@internal a, b ; c, d, e ; e) // now only one elem in counter
// - balanced_or_tree!(a, b, c).or(balanced_or_tree(d, e)) // recurse on each sublist
#[macro_export]
macro_rules! balanced_or_tree {
    // Base case: just a single expression, return it wrapped in `debug_boxed`
    ($x:expr $(,)?) => { crate::debug_boxed!($x) };
    // Multiple expressions: recurse with three lists: left, right and counter.
    ($($x:expr),+ $(,)?) => {
        balanced_or_tree!(@internal  ;     $($x),+; $($x),+)
        //                          ^ left ^ right  ^ counter
    };
    // Counter 1 or 2; move one more item and recurse on each sublist, and or them together
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr $(,$b:expr)?) => {
        (balanced_or_tree!($($left,)* $head)).or(balanced_or_tree!($($tail),+))
    };
    // Counter > 2; move one item from the right to the left and subtract two from the counter
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr, $b:expr, $($more:expr),+) => {
        balanced_or_tree!(@internal $($left,)* $head; $($tail),+; $($more),+)
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_boxed {
    ($x:expr) => {
        ::warp::Filter::boxed($x)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_boxed {
    ($x:expr) => {
        $x
    };
}
