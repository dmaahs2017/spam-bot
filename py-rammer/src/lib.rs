use cpython::{PyResult, Python, py_module_initializer, py_fn, py_class};
use rammer::HSModel; 
use rammer::Probability;


py_class!(class PyHSModel |py| {
    data model: HSModel;
    def __new__(_cls, saved_model_path: &str) -> PyResult<PyHSModel> {
        let model = HSModel::read_from_json(saved_model_path).expect("Could not find model");
        PyHSModel::create_instance(py, model)
    }
    def predict_on_text(&self, text: &str) -> PyResult<Probability> {
        Ok(self.model(py).text_spam_probability(text))
    }
});

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(pyrammer, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "load_hs_model", py_fn!(py, load_hs_model(json_path: &str)))?;
    Ok(())
});

// logic implemented as a normal rust function
fn load_hs_model(_: Python, json_path: &str) -> PyResult<PyHSModel> {
    let py = Python::acquire_gil();
    PyHSModel::create_instance(py.python(), HSModel::read_from_json(json_path).expect("Model not found"))
}
