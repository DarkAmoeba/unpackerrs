Starting point:

(ArjanCodes)(https://www.youtube.com/watch?v=lyG6AKzu4ew)

Needed to install rust and maturin.  pyo3 is a rust module and becomes available via the Cargo toml file.

`pip install maturin`

`maturin init` - to crete a project

`matturin develop` - to build

The very first function is now working and can be tested as follows:
```python
Python 3.11.8 (main, Feb 26 2024, 21:39:34) [GCC 11.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import unpackerrs
>>> unpackerrs.sum_as_string(3,4)
'7'
>>>
```
