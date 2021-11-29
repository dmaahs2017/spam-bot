from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyrammer",
    version="0.0.1",
    description="This is a wrapper over my rust library called rammer. https://github.com/dmaahs2017/rammer",
    rust_extensions=[RustExtension("pyrammer", binding=Binding.RustCPython)],
    packages=[],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
