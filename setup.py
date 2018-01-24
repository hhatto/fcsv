from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(name='fcsv',
      version='0.1',
      rust_extensions=[
          RustExtension('fcsv', 'Cargo.toml', binding=Binding.PyO3)],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
