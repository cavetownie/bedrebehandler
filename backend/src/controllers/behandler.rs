use super::*;

use crate::handlers;

pub async fn list(req: Request<State>) -> tide::Result {
    let db = req.state().db_pool.clone();

    let behandlere = handlers::behandler::list(db).await;

    match behandlere {
        Ok(result) => {
            let mut res = tide::Response::new(tide::StatusCode::Ok);
            res.set_body(serde_json::to_string(&result)?);

            Ok(res)
        }
        // Propogate server error from handler
        Err(err) => Err(err)
    }
}

pub async fn get_by_type(req: Request<State>) -> tide::Result {
    let db = req.state().db_pool.clone();
    let behandler_type = req.param("kliniktype")?;

    let behandlere_by_type = handlers::behandler::get_by_type(behandler_type, db).await;

    match behandlere_by_type {
        Ok(result) => {
            let mut res = tide::Response::new(tide::StatusCode::Ok);
            res.set_body(serde_json::to_string(&result)?);

            Ok(res)
        }
        // Propogate server error from handler
        Err(err) => Err(err)
    }
}

pub async fn get_by_type_and_open(req: Request<State>) -> tide::Result {
    let db = req.state().db_pool.clone();
    let behandler_type = req.param("kliniktype")?;

    let behandlere_by_type = handlers::behandler::get_by_type_and_open(behandler_type, db).await;

    match behandlere_by_type {
        Ok(result) => {
            let mut res = tide::Response::new(tide::StatusCode::Ok);
            res.set_body(serde_json::to_string(&result)?);

            Ok(res)
        }
        // Propogate server error from handler
        Err(err) => Err(err)
    }
}
