use polars::lazy::dsl;
use polars::prelude::*;
use wasm_bindgen::prelude::*;

use crate::JsResult;

use super::expr::JsExpr;

#[wasm_bindgen]
#[derive(Clone)]
pub struct When {
    predicate: Expr,
}
#[wasm_bindgen]
#[derive(Clone)]
pub struct WhenThen {
    predicate: Expr,
    then: Expr,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WhenThenThen {
    inner: dsl::WhenThenThen,
}

#[wasm_bindgen]
impl When {
    pub fn then(&self, expr: &JsExpr) -> WhenThen {
        WhenThen {
            predicate: self.predicate.clone(),
            then: expr.inner.clone(),
        }
    }
}
#[wasm_bindgen]
impl WhenThen {
    pub fn when(&self, predicate: &JsExpr) -> WhenThenThen {
        let e = dsl::when(self.predicate.clone())
            .then(self.then.clone())
            .when(predicate.inner.clone());
        WhenThenThen { inner: e }
    }

    pub fn otherwise(&self, expr: &JsExpr) -> JsExpr {
        dsl::ternary_expr(
            self.predicate.clone(),
            self.then.clone(),
            expr.inner.clone(),
        )
        .into()
    }
}

#[wasm_bindgen]
impl WhenThenThen {
    pub fn when(&self, predicate: &JsExpr) -> WhenThenThen {
        Self {
            inner: self.inner.clone().when(predicate.inner.clone()),
        }
    }

    pub fn then(&self, expr: &JsExpr) -> WhenThenThen {
        Self {
            inner: self.inner.clone().then(expr.inner.clone()),
        }
    }
    pub fn otherwise(&self, expr: &JsExpr) -> JsExpr {
        self.inner.clone().otherwise(expr.inner.clone()).into()
    }
}

#[wasm_bindgen]
pub fn when(predicate: &JsExpr) -> When {
    When {
        predicate: predicate.inner.clone(),
    }
}

///  __A column in a DataFrame.__
///  Can be used to select:
///    - a single column by name
///    - all columns by using a wildcard `"*"`
///    - column by regular expression if the regex starts with `^` and ends with `$`
///  @param col
///  @example
///  ```js
///  > df = pl.DataFrame({
///  ... "ham": [1, 2, 3],
///  ... "hamburger": [11, 22, 33],
///  ... "foo": [3, 2, 1]})
///  > df.select(col("foo"))
///  shape: (3, 1)
///  ╭─────╮
///  │ foo │
///  │ --- │
///  │ i64 │
///  ╞═════╡
///  │ 3   │
///  ├╌╌╌╌╌┤
///  │ 2   │
///  ├╌╌╌╌╌┤
///  │ 1   │
///  ╰─────╯
///  > df.select(col("*"))
///  shape: (3, 3)
///  ╭─────┬───────────┬─────╮
///  │ ham ┆ hamburger ┆ foo │
///  │ --- ┆ ---       ┆ --- │
///  │ i64 ┆ i64       ┆ i64 │
///  ╞═════╪═══════════╪═════╡
///  │ 1   ┆ 11        ┆ 3   │
///  ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 2   ┆ 22        ┆ 2   │
///  ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 3   ┆ 33        ┆ 1   │
///  ╰─────┴───────────┴─────╯
///  > df.select(col("^ham.*$"))
///  shape: (3, 2)
///  ╭─────┬───────────╮
///  │ ham ┆ hamburger │
///  │ --- ┆ ---       │
///  │ i64 ┆ i64       │
///  ╞═════╪═══════════╡
///  │ 1   ┆ 11        │
///  ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
///  │ 2   ┆ 22        │
///  ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
///  │ 3   ┆ 33        │
///  ╰─────┴───────────╯
///  > df.select(col("*").exclude("ham"))
///  shape: (3, 2)
///  ╭───────────┬─────╮
///  │ hamburger ┆ foo │
///  │ ---       ┆ --- │
///  │ i64       ┆ i64 │
///  ╞═══════════╪═════╡
///  │ 11        ┆ 3   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 22        ┆ 2   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 33        ┆ 1   │
///  ╰───────────┴─────╯
///  > df.select(col(["hamburger", "foo"])
///  shape: (3, 2)
///  ╭───────────┬─────╮
///  │ hamburger ┆ foo │
///  │ ---       ┆ --- │
///  │ i64       ┆ i64 │
///  ╞═══════════╪═════╡
///  │ 11        ┆ 3   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 22        ┆ 2   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 33        ┆ 1   │
///  ╰───────────┴─────╯
///  > df.select(col(pl.Series(["hamburger", "foo"]))
///  shape: (3, 2)
///  ╭───────────┬─────╮
///  │ hamburger ┆ foo │
///  │ ---       ┆ --- │
///  │ i64       ┆ i64 │
///  ╞═══════════╪═════╡
///  │ 11        ┆ 3   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 22        ┆ 2   │
///  ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┤
///  │ 33        ┆ 1   │
///  ╰───────────┴─────╯
///  ```
/// /
#[wasm_bindgen]
pub fn col(name: String) -> JsExpr {
    dsl::col(&name).into()
}

#[wasm_bindgen]
pub fn count() -> JsExpr {
    dsl::count().into()
}

#[wasm_bindgen]
pub fn first() -> JsExpr {
    dsl::first().into()
}

#[wasm_bindgen]
pub fn last() -> JsExpr {
    dsl::last().into()
}

#[wasm_bindgen]
pub fn cols(names: JsValue) -> JsExpr {
    let names: Vec<String> = serde_wasm_bindgen::from_value(names).unwrap();
    dsl::cols(names).into()
}

#[wasm_bindgen]
pub fn lit(value: JsValue) -> JsResult<JsExpr> {
    use wasm_bindgen::JsCast;
    if value.is_bigint() {
        let value: js_sys::BigInt = value.dyn_into().unwrap();
        let value: f64 = value.as_f64().unwrap();
        return Ok(value.lit().into());
    } else if value.is_null() | value.is_undefined() {
        Ok(dsl::lit(Null {}).into())
    } else if value.is_string() {
        let value: String = value.as_string().unwrap();
        return Ok(value.lit().into());
    } else if let Some(bool) = value.as_bool() {
        return Ok(bool.lit().into());
    } else if let Some(num) = value.as_f64() {
        return Ok(num.lit().into());
    } else if js_sys::Array::is_array(&value) {
        todo!("array")
    } else {
        todo!("others")
    }
}
