use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use namedivider_rs::divider::divided_name::DividedName;
use namedivider_rs::divider::basic_name_divider::BasicNameDivider;
use namedivider_rs::divider::basic_name_divider::get_basic_name_divider;
use namedivider_rs::divider::gbdt_name_divider::GBDTNameDivider;
use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;
use namedivider_rs::divider::score_calculator::ScoreCalculator;

#[pyclass(name="DividedName")]
struct PyDividedName{
    family: String,
    given: String,
    separator: String,
    algorithm: String,
    score: f64
}

#[pymethods]
impl PyDividedName {
    #[getter]
    fn family(&self) -> PyResult<String>{
        Ok(self.family.clone())
    }

    #[getter]
    fn given(&self) -> PyResult<String>{
        Ok(self.given.clone())
    }

    #[getter]
    fn separator(&self) -> PyResult<String>{
        Ok(self.separator.clone())
    }

    #[getter]
    fn algorithm(&self) -> PyResult<String>{
        Ok(self.algorithm.clone())
    }

    #[getter]
    fn score(&self) -> PyResult<f64>{
        Ok(self.score)
    }

    fn __str__(&self) -> PyResult<String> {
        return Ok(self.family.clone() + &self.separator + &self.given)
    }

    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>>{
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("family", self.family.to_object(py)),
            ("given", self.given.to_object(py)),
            ("separator", self.separator.to_object(py)),
            ("algorithm", self.algorithm.to_object(py)),
            ("score", self.score.to_object(py))
        ];
        let dict = key_vals.into_py_dict(py);
        return Ok(dict.into());
    }
}


#[pyclass(name="BasicNameDivider")]
struct PyBasicNameDivider{
    divider: BasicNameDivider
}

#[pymethods]
impl PyBasicNameDivider{
    #[new]
    #[args(separator="\" \"",normalize_name=true, only_order_score_when_4=false)]
    fn new(separator: &str, normalize_name: bool, only_order_score_when_4: bool) -> Self {
        let divider = get_basic_name_divider(separator.to_string(),
                                             normalize_name,
                                             "kanji_feature".to_string(),
                                             only_order_score_when_4);
        return Self{divider};
    }

    fn calc_score(&self, family: String, given: String) -> PyResult<f64> {
        return Ok(self.divider.basic_score_calculator.calc_score(&family, &given));
    }

    fn divide_name(&self, undivided_name: String) -> PyResult<PyDividedName>{
        let divided_name = self.divider.divide_name(&undivided_name);
        return Ok(PyDividedName{
            family: divided_name.family,
            given: divided_name.given,
            separator: divided_name.separator,
            algorithm: divided_name.algorithm,
            score: divided_name.score
        });
    }
}

#[pyclass(unsendable, name="GBDTNameDivider")]
struct PyGBDTNameDivider{
    divider: GBDTNameDivider
}

#[pymethods]
impl PyGBDTNameDivider{
    #[new]
    #[args(separator="\" \"",normalize_name=true, only_order_score_when_4=false)]
    fn new(separator: &str, normalize_name: bool, only_order_score_when_4: bool) -> Self {
        let divider = get_gbdt_name_divider(separator.to_string(),
                                             normalize_name,
                                             "gbdt".to_string());
        return Self{divider};
    }

    fn calc_score(&self, family: String, given: String) -> PyResult<f64> {
        return Ok(self.divider.gbdt_score_calculator.calc_score(&family, &given));
    }

    fn divide_name(&self, undivided_name: String) -> PyResult<PyDividedName>{
        let divided_name = self.divider.divide_name(&undivided_name);
        return Ok(PyDividedName{
            family: divided_name.family,
            given: divided_name.given,
            separator: divided_name.separator,
            algorithm: divided_name.algorithm,
            score: divided_name.score
        });
    }
}

#[pymodule]
fn namedivider(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDividedName>()?;
    m.add_class::<PyBasicNameDivider>()?;
    m.add_class::<PyGBDTNameDivider>()?;
    Ok(())
}
