use pyo3::prelude::*;
use pyo3::types::PyDict;

use namedivider_rs::divider::basic_name_divider::get_basic_name_divider;
use namedivider_rs::divider::basic_name_divider::BasicNameDivider;
use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::gbdt_name_divider::GBDTNameDivider;
use namedivider_rs::divider::name_divider::NameDivider;
use namedivider_rs::divider::score_calculator::ScoreCalculator;

#[pyclass(name = "DividedName")]
struct PyDividedName {
    family: String,
    given: String,
    separator: String,
    algorithm: String,
    score: f64,
}

#[pymethods]
impl PyDividedName {
    #[getter]
    fn family(&self) -> PyResult<String> {
        Ok(self.family.clone())
    }

    #[getter]
    fn given(&self) -> PyResult<String> {
        Ok(self.given.clone())
    }

    #[getter]
    fn separator(&self) -> PyResult<String> {
        Ok(self.separator.clone())
    }

    #[getter]
    fn algorithm(&self) -> PyResult<String> {
        Ok(self.algorithm.clone())
    }

    #[getter]
    fn score(&self) -> PyResult<f64> {
        Ok(self.score)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.family.clone() + &self.separator + &self.given)
    }

    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("family", &self.family)?;
        dict.set_item("given", &self.given)?;
        dict.set_item("separator", &self.separator)?;
        dict.set_item("algorithm", &self.algorithm)?;
        dict.set_item("score", self.score)?;
        Ok(dict.unbind())
    }
}

#[pyclass(name = "BasicNameDivider")]
struct PyBasicNameDivider {
    divider: BasicNameDivider,
}

#[pymethods]
impl PyBasicNameDivider {
    #[new]
    #[pyo3(signature = (separator = " ", normalize_name = true, only_order_score_when_4 = false))]
    fn new(separator: &str, normalize_name: bool, only_order_score_when_4: bool) -> Self {
        let divider = get_basic_name_divider(
            separator.to_string(),
            normalize_name,
            "kanji_feature".to_string(),
            only_order_score_when_4,
        );
        Self { divider }
    }

    fn calc_score(&self, family: String, given: String) -> PyResult<f64> {
        Ok(self
            .divider
            .basic_score_calculator
            .calc_score(&family, &given))
    }

    fn divide_name(&self, undivided_name: String) -> PyResult<PyDividedName> {
        let divided_name = self.divider.divide_name(&undivided_name);
        Ok(PyDividedName {
            family: divided_name.family,
            given: divided_name.given,
            separator: divided_name.separator,
            algorithm: divided_name.algorithm,
            score: divided_name.score,
        })
    }

    fn divide_names(&self, undivided_names: Vec<String>) -> PyResult<Vec<PyDividedName>> {
        let mut results = Vec::new();
        for undivided_name in undivided_names {
            let divided_name = self.divider.divide_name(&undivided_name);
            results.push(PyDividedName {
                family: divided_name.family,
                given: divided_name.given,
                separator: divided_name.separator,
                algorithm: divided_name.algorithm,
                score: divided_name.score,
            });
        }
        Ok(results)
    }
}

#[pyclass(name = "GBDTNameDivider")]
struct PyGBDTNameDivider {
    divider: GBDTNameDivider,
}

#[pymethods]
impl PyGBDTNameDivider {
    #[new]
    #[pyo3(signature = (separator = " ", normalize_name = true))]
    fn new(separator: &str, normalize_name: bool) -> Self {
        let divider =
            get_gbdt_name_divider(separator.to_string(), normalize_name, "gbdt".to_string());
        Self { divider }
    }

    fn calc_score(&self, family: String, given: String) -> PyResult<f64> {
        Ok(self
            .divider
            .gbdt_score_calculator
            .calc_score(&family, &given))
    }

    fn divide_name(&self, undivided_name: String) -> PyResult<PyDividedName> {
        let divided_name = self.divider.divide_name(&undivided_name);
        Ok(PyDividedName {
            family: divided_name.family,
            given: divided_name.given,
            separator: divided_name.separator,
            algorithm: divided_name.algorithm,
            score: divided_name.score,
        })
    }

    fn divide_names(&self, undivided_names: Vec<String>) -> PyResult<Vec<PyDividedName>> {
        let mut results = Vec::new();
        for undivided_name in undivided_names {
            let divided_name = self.divider.divide_name(&undivided_name);
            results.push(PyDividedName {
                family: divided_name.family,
                given: divided_name.given,
                separator: divided_name.separator,
                algorithm: divided_name.algorithm,
                score: divided_name.score,
            });
        }
        Ok(results)
    }
}

#[pymodule]
fn namedivider_core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    
    m.add_class::<PyDividedName>()?;
    m.add_class::<PyBasicNameDivider>()?;
    m.add_class::<PyGBDTNameDivider>()?;
    
    // Add version information from Cargo.toml
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    Ok(())
}
