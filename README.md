# data-from

A fairly opinionated tool that provides the output of commands that typically
need post-processing with sed, grep as structured data. Inspired loosely by
Powershell and Nushell, and generally by the idea that stringly-typed data can
make life harder than it needs to be.

Provides output as _ndjson_ ("Newline-delimited JSON"), which is also known as the
_json-lines_ format.

## Status

Just a prototype at this stage. Seeking feedback.

## Usage and operation

Generally speaking, you either pipe the output of the intended command (with the
necessary arguments to enable parsing by machine) and provide the source
prefixed with a double-dash, or provide the command's name as the first argument.

For example, consider `lspci`. The following two forms produce equivalent output:

    data-from lspci
    lspci -vmm -nn | data-from --lspci

Note: Failing to provide the information in the intended way will cause
`data-from` to fail. For example, omitting the `-vmm -nn` flags from `lspci`
means that `data-from` won't be able to parse the output correctly.

When you provide a command's name, rather than sending data to stdin,
`data-from` will invoke the command on its own within a sub-process.

### lspci

To list PCI interfaces, you can either request that `data-from` invokes `lspci`
itself (`data-from lspci`), or you can pipe data into `data-from` (`lspci -vmm -nn | data-from --lspci`).

Example output (with some formatting applied):

```console
$ data-from lspci
...
{
  "slot": "6c:00.0",
  "device": "RTS5260 PCI Express Card Reader",
  "device-code": { "hex": ["52", "60"], "int": [82, 96] },
  "sub-device": "RTS5260 PCI Express Card Reader",
  "sub-device-code": { "hex": ["09", "7d"], "int": [9, 125] },
  "class":"Unassigned class",
  "class-code": { "hex": ["ff", "00"], "int": [255, 0] },
  "vendor":"Realtek Semiconductor Co., Ltd.",
  "vendor-code": { "hex": ["10", "ec"], "int": [16, 236] },
  "sub-vendor": "Dell",
  "sub-vendor-code": { "hex": ["10", "28"], "int": [16, 40] },
  "revision": 1,
  "programming-interface": null
}
```

The `*-code` values are the numbers supplied by the devices themselves. To match
these codes to names, your computer typically makes a lookup to the [PCI ID
Repository](https://pci-ids.ucw.cz/).

## Alternatives

- "grep and hope"
- `jc` is a Python tool which is much more complete. It requires a Python
  interpreter and only produces JSON. The JSON output is typically wrapped in an
  array, which makes it difficult for follow-on tools which expect input line-by-line.
