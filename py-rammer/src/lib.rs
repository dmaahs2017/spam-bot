use cpython::{PyResult, Python, py_module_initializer, py_fn, py_class, PyNone};
use rammer::{HSModel, BagOfWords, Probability, Count, Frequency}; 

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(pyrammer, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "hs_model_read_from_json", py_fn!(py, hs_model_read_from_json(json_path: &str)))?;
    m.add(py, "hs_model_new", py_fn!(py, new_hs_model()))?;
    m.add(py, "hs_model_from_bows", py_fn!(py, hs_model_from_bows(ham_bow: PyBagOfWords, spam_bow: PyBagOfWords)))?;
    m.add(py, "bow_new", py_fn!(py, bow_new()))?;
    m.add(py, "bow_from_file", py_fn!(py, bow_from_file(file_path: &str)))?;
    m.add(py, "bow_from_folder", py_fn!(py, bow_from_folder(folder_path: &str)))?;
    Ok(())
});

py_class!(class PyBagOfWords |py| {
    data bow: BagOfWords;
    def combine(&self, other: &PyBagOfWords) -> PyResult<PyBagOfWords> {
        let lhs = self.bow(py).clone();
        let rhs = other.bow(py).clone();
        PyBagOfWords::create_instance(py, lhs.combine(rhs))
    }
    def total_word_count(&self) -> PyResult<Count> {
        Ok(self.bow(py).total_word_count())
    }
    def word_frequency(&self, word: &str) -> PyResult<Option<Frequency>> {
        Ok(self.bow(py).word_frequency(word))
    }

});

py_class!(class PyHSModel |py| {
    data model: HSModel;    
    def predict_on_text(&self, text: &str) -> PyResult<Probability> {
        Ok(self.model(py).text_spam_probability(text))
    }
    def add_spam_bow(&self, bow: PyBagOfWords) -> PyResult<PyHSModel> {
        PyHSModel::create_instance(py, self.model(py).clone().add_spam_bow(bow.bow(py).clone()))
    }
    def add_ham_bow(&self, bow: PyBagOfWords) -> PyResult<PyHSModel> {
        PyHSModel::create_instance(py, self.model(py).clone().add_ham_bow(bow.bow(py).clone()))
    }
    def write_to_json(&self, file_path: &str) -> PyResult<PyNone> {
        self.model(py).write_to_json(file_path).expect(&format!("Failed writing to file: {}", file_path));
        Ok(PyNone)
    }
});


// logic implemented as a normal rust function
fn hs_model_read_from_json(_: Python, json_path: &str) -> PyResult<PyHSModel> {
    let py = Python::acquire_gil();
    PyHSModel::create_instance(py.python(), HSModel::read_from_json(json_path).expect("Model not found"))
}

// logic implemented as a normal rust function
fn new_hs_model(_: Python) -> PyResult<PyHSModel> {
    let py = Python::acquire_gil();
    PyHSModel::create_instance(py.python(), HSModel::new())
}

fn hs_model_from_bows(_: Python, ham_bow: PyBagOfWords, spam_bow: PyBagOfWords) -> PyResult<PyHSModel> {
    let lock = Python::acquire_gil();
    let py = lock.python();
    let ham_bow = ham_bow.bow(py).clone();
    let spam_bow = spam_bow.bow(py).clone();
    PyHSModel::create_instance(py, HSModel::from_bows(ham_bow, spam_bow))
}

fn bow_new(_: Python) -> PyResult<PyBagOfWords> {
    let py = Python::acquire_gil();
    PyBagOfWords::create_instance(py.python(), BagOfWords::new())
}

fn bow_from_file(_: Python, file_path: &str) -> PyResult<PyBagOfWords> {
    let py = Python::acquire_gil();
    PyBagOfWords::create_instance(py.python(), BagOfWords::from_file(file_path).expect(&format!("File not found: {}", file_path)))
}

fn bow_from_folder(_: Python, folder_path: &str) -> PyResult<PyBagOfWords> {
    let py = Python::acquire_gil();
    PyBagOfWords::create_instance(py.python(), BagOfWords::from_folder(folder_path).expect(&format!("Folder not found: {}", folder_path)))
}
