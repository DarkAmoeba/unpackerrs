Starting point:

(ArjanCodes)(https://www.youtube.com/watch?v=lyG6AKzu4ew)

Needed to install rust and maturin.  pyo3 is a rust module and becomes available via the Cargo toml file.

`pip install maturin`

`maturin init` - to crete a project

`matturin develop` - to build

The very first function is now working and can be tested as follows:
## Usage

Using ChatGPT I have updated this dump to binary and go back the other way.  This now needs changing to make use of the packed-struct library which allows specification of the correct bits.
When compiling I moved the `unpackerrs` directory out of the way so that the defualt libraries would appear

```python
Python 3.11.8 (main, Feb 26 2024, 21:39:34) [GCC 11.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import unpackerrs
>>> dir(unpackerrs)
['__all__', '__builtins__', '__cached__', '__doc__', '__file__', '__loader__', '__name__', '__package__', '__path__', '__spec__', 'deserialize_from_binary', 'serialize_to_binary', 'unpackerrs']
>>> json_str = '{"name": "Alice", "age": 30}'
>>> binary_data = unpackerrs.serialize_to_binary(json_str)
>>> print(binary_data)
[5, 0, 0, 0, 0, 0, 0, 0, 65, 108, 105, 99, 101, 30, 0, 0, 0]
>>> reconstructed_json_str = unpackerrs.deserialize_from_binary(binary_data)
>>> print(reconstructed_json_str)
{"name":"Alice","age":30}
>>>
```
